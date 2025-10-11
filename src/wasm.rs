use wasm_bindgen::prelude::*;
use crate::board::Board;
use crate::dictionary::Dictionary;
use crate::solver::Solver;
use std::sync::OnceLock;
use std::sync::atomic::{AtomicU32, Ordering};

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

// Global dictionary storage
static GLOBAL_DICTIONARY: OnceLock<Dictionary> = OnceLock::new();

// Global cancellation generation counter
static CURRENT_GENERATION: AtomicU32 = AtomicU32::new(0);

#[wasm_bindgen]
pub fn initialize_dictionary(dictionary_data: Vec<u8>) -> Result<(), String> {
    console_log!("Initializing global dictionary from {} bytes", dictionary_data.len());

    let dictionary = Dictionary::from_bytes(&dictionary_data)?;
    console_log!("Parsed dictionary with {} words", dictionary.words.len());

    match GLOBAL_DICTIONARY.set(dictionary) {
        Ok(()) => {
            console_log!("Global dictionary initialized successfully");
            Ok(())
        }
        Err(_) => Err("Dictionary already initialized".to_string())
    }
}

#[wasm_bindgen]
pub fn solve_game(game_sides: Vec<String>, max_solutions: u16) -> Vec<String> {
    console_log!("Solving game with {} sides", game_sides.len());

    // Check if dictionary is initialized
    let dictionary = match GLOBAL_DICTIONARY.get() {
        Some(dict) => dict,
        None => {
            console_log!("Error: Dictionary not initialized");
            return vec!["Error: Dictionary not initialized. Call initialize_dictionary first.".to_string()];
        }
    };

    // Create the board from the provided sides
    let board = match Board::from_sides(game_sides) {
        Ok(board) => board,
        Err(e) => {
            console_log!("Error creating board: {}", e);
            return vec![format!("Error: {}", e)];
        }
    };

    // Create solver and solve using the global dictionary
    let solver = Solver::new(board, dictionary, max_solutions);
    let solutions = solver.solve();

    console_log!("Found {} solutions", solutions.len());

    // Convert solutions to strings with score appended
    solutions.iter().map(|s| format!("{}:{}", s.to_string(), s.score)).collect()
}

#[wasm_bindgen]
pub fn cancel_current_solve() {
    CURRENT_GENERATION.fetch_add(1, Ordering::SeqCst);
    console_log!("Cancellation requested, generation now: {}", CURRENT_GENERATION.load(Ordering::SeqCst));
}

#[wasm_bindgen]
pub fn solve_game_streaming(
    game_sides: Vec<String>,
    max_solutions: u16,
    callback: js_sys::Function,
) -> u32 {
    let generation = CURRENT_GENERATION.load(Ordering::SeqCst);
    console_log!("Starting solve with generation: {}", generation);

    // Check if dictionary is initialized
    let dictionary = match GLOBAL_DICTIONARY.get() {
        Some(dict) => dict,
        None => {
            console_log!("Error: Dictionary not initialized");
            return 0;
        }
    };

    // Create the board from the provided sides
    let board = match Board::from_sides(game_sides) {
        Ok(board) => board,
        Err(e) => {
            console_log!("Error creating board: {}", e);
            return 0;
        }
    };

    // Create solver and solve using the global dictionary with streaming callback
    let solver = Solver::new(board, dictionary, max_solutions);

    let count = solver.solve_streaming(|solution_batch| {
        // Check if we've been cancelled before processing this batch
        if CURRENT_GENERATION.load(Ordering::SeqCst) != generation {
            console_log!("Solve cancelled at generation {} (current: {})",
                generation, CURRENT_GENERATION.load(Ordering::SeqCst));
            return false; // Stop solving
        }

        // Convert batch to JS array of strings with scores
        let js_array = js_sys::Array::new();
        for solution in solution_batch {
            let solution_str = format!("{}:{}", solution.to_string(), solution.score);
            js_array.push(&JsValue::from_str(&solution_str));
        }

        // Send batch back to JavaScript
        let this = JsValue::null();
        let _ = callback.call1(&this, &js_array);

        true // Continue solving
    }, generation);

    console_log!("Solve completed, found {} solutions", count);
    count as u32
}