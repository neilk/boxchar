use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use crate::board::Board;
use crate::dictionary::Dictionary;
use crate::solver::Solver;
use std::sync::{OnceLock, Mutex};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use js_sys::Promise;

// Import the `console.log` function from the browser's Web API
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

// Define a macro to make console logging easier
macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

// Global dictionary storage (wrapped in Arc for sharing across tasks)
static GLOBAL_DICTIONARY: OnceLock<Arc<Dictionary>> = OnceLock::new();

// Current solve task state
#[derive(Clone, PartialEq)]
struct SolveParams {
    sides: Vec<String>,
    max_solutions: u16,
}

struct SolveTask {
    params: SolveParams,
    cancel_flag: Arc<AtomicBool>,
}

static CURRENT_SOLVE: OnceLock<Mutex<Option<SolveTask>>> = OnceLock::new();

#[wasm_bindgen]
pub fn initialize_dictionary(dictionary_data: Vec<u8>) -> Result<(), String> {
    console_log!("Initializing global dictionary from {} bytes", dictionary_data.len());

    let dictionary = Dictionary::from_bytes(&dictionary_data)?;
    console_log!("Parsed dictionary with {} words", dictionary.words.len());

    // Initialize the current solve tracker
    let _ = CURRENT_SOLVE.set(Mutex::new(None));

    match GLOBAL_DICTIONARY.set(Arc::new(dictionary)) {
        Ok(()) => {
            console_log!("Global dictionary initialized successfully");
            Ok(())
        }
        Err(_) => Err("Dictionary already initialized".to_string())
    }
}

#[wasm_bindgen]
pub fn solve_game(game_sides: Vec<String>, max_solutions: u16) -> Promise {
    console_log!("Solve requested with {} sides", game_sides.len());

    // Create a promise to return
    let promise = Promise::new(&mut |resolve, reject| {
        // Check if dictionary is initialized
        let dictionary = match GLOBAL_DICTIONARY.get() {
            Some(dict) => dict,
            None => {
                console_log!("Error: Dictionary not initialized");
                let error = JsValue::from_str("Dictionary not initialized");
                reject.call1(&JsValue::null(), &error).unwrap();
                return;
            }
        };

        let new_params = SolveParams {
            sides: game_sides.clone(),
            max_solutions,
        };

        // Check if we need to cancel an existing solve
        let cancel_flag = if let Some(solve_mutex) = CURRENT_SOLVE.get() {
            let mut current = solve_mutex.lock().unwrap();

            // If there's a current task with different params, cancel it
            if let Some(ref task) = *current {
                if task.params != new_params {
                    console_log!("Cancelling previous solve with different params");
                    task.cancel_flag.store(true, Ordering::Relaxed);
                } else {
                    console_log!("Solve already in progress with same params, rejecting duplicate");
                    let error = JsValue::from_str("Solve already in progress");
                    reject.call1(&JsValue::null(), &error).unwrap();
                    return;
                }
            }

            // Create new cancel flag and task
            let cancel_flag = Arc::new(AtomicBool::new(false));
            *current = Some(SolveTask {
                params: new_params.clone(),
                cancel_flag: cancel_flag.clone(),
            });

            cancel_flag
        } else {
            console_log!("Error: CURRENT_SOLVE not initialized");
            let error = JsValue::from_str("Solver not initialized");
            reject.call1(&JsValue::null(), &error).unwrap();
            return;
        };

        // Create the board (clone game_sides since we need it later)
        let board = match Board::from_sides(game_sides.clone()) {
            Ok(board) => board,
            Err(e) => {
                console_log!("Error creating board: {}", e);
                let error = JsValue::from_str(&e.to_string());
                reject.call1(&JsValue::null(), &error).unwrap();

                // Clear current task since we failed
                if let Some(solve_mutex) = CURRENT_SOLVE.get() {
                    *solve_mutex.lock().unwrap() = None;
                }
                return;
            }
        };

        // Clone the Arc (cheap) for the async task
        let dictionary_arc = dictionary.clone();

        // Spawn the solve task
        spawn_local(async move {
            console_log!("Starting solve task");

            let solver = Solver::new(board, &dictionary_arc, max_solutions);
            let solutions = solver.solve_cancellable(Some(cancel_flag.clone()));

            // Check if we were cancelled
            if cancel_flag.load(Ordering::Relaxed) {
                console_log!("Solve was cancelled");
                let error = JsValue::from_str("Cancelled");
                let _ = reject.call1(&JsValue::null(), &error);
            } else {
                console_log!("Found {} solutions", solutions.len());

                // Convert solutions to JS array
                let js_array = js_sys::Array::new();
                for solution in &solutions {
                    let solution_str = format!("{}:{}", solution.to_string(), solution.score);
                    js_array.push(&JsValue::from_str(&solution_str));
                }

                // Resolve the promise with the results
                let _ = resolve.call1(&JsValue::null(), &js_array);
            }

            // Clear current task
            if let Some(solve_mutex) = CURRENT_SOLVE.get() {
                let mut current = solve_mutex.lock().unwrap();
                // Only clear if this is still our task (not replaced by a new one)
                if let Some(ref task) = *current {
                    if Arc::ptr_eq(&task.cancel_flag, &cancel_flag) {
                        *current = None;
                    }
                }
            }
        });

        console_log!("Solve task spawned");
    });

    promise
}

#[wasm_bindgen]
pub fn cancel_current_solve() {
    if let Some(solve_mutex) = CURRENT_SOLVE.get() {
        let mut current = solve_mutex.lock().unwrap();
        if let Some(ref task) = *current {
            console_log!("Cancelling current solve");
            task.cancel_flag.store(true, Ordering::Relaxed);
            *current = None;
        } else {
            console_log!("No solve in progress to cancel");
        }
    } else {
        console_log!("Warning: Solver not initialized");
    }
}