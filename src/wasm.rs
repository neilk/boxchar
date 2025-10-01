use wasm_bindgen::prelude::*;
use crate::board::Board;
use crate::dictionary::Dictionary;
use crate::solver::Solver;

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

#[wasm_bindgen]
pub fn solve_game(game_sides: Vec<String>, dictionary_text: String, max_solutions: u16) -> Vec<String> {
    console_log!("Solving game with {} sides and dictionary text of {} chars", game_sides.len(), dictionary_text.len());

    // Create the board from the provided sides
    let board = match Board::from_sides(game_sides) {
        Ok(board) => board,
        Err(e) => {
            console_log!("Error creating board: {}", e);
            return vec![format!("Error: {}", e)];
        }
    };

    // Create dictionary from the provided text (word frequency format)
    let dictionary = Dictionary::from_text(&dictionary_text);
    console_log!("Loaded {} words from dictionary", dictionary.words.len());

    // Create solver and solve
    let solver = Solver::new(board, dictionary, max_solutions);
    let solutions = solver.solve();

    console_log!("Found {} solutions", solutions.len());

    // Convert solutions to strings using the Display trait
    solutions.iter().map(|s| s.to_string()).collect()
}

#[wasm_bindgen]
pub fn solve_game_with_bytes(game_sides: Vec<String>, dictionary_data: Vec<u8>, max_solutions: u16) -> Vec<String> {
    console_log!("Solving game with {} sides and dictionary data of {} bytes", game_sides.len(), dictionary_data.len());

    // Create the board from the provided sides
    let board = match Board::from_sides(game_sides) {
        Ok(board) => board,
        Err(e) => {
            console_log!("Error creating board: {}", e);
            return vec![format!("Error: {}", e)];
        }
    };

    // Create dictionary from the provided binary data
    let dictionary = match Dictionary::from_bytes(&dictionary_data) {
        Ok(dict) => dict,
        Err(e) => {
            console_log!("Error parsing dictionary: {}", e);
            return vec![format!("Error: {}", e)];
        }
    };
    console_log!("Loaded {} words from dictionary", dictionary.words.len());

    // Create solver and solve
    let solver = Solver::new(board, dictionary, max_solutions);
    let solutions = solver.solve();

    console_log!("Found {} solutions", solutions.len());

    // Convert solutions to strings using the Display trait
    solutions.iter().map(|s| s.to_string()).collect()
}