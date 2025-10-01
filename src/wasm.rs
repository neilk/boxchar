use wasm_bindgen::prelude::*;
use crate::board::Board;
use crate::dictionary::Dictionary;
use crate::solver::Solver;
use std::sync::OnceLock;

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

    // Convert solutions to strings using the Display trait
    solutions.iter().map(|s| s.to_string()).collect()
}