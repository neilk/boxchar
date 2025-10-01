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
pub fn solve_game(game_sides: Vec<String>, wordlist_words: Vec<String>, max_solutions: u16) -> Vec<String> {
    console_log!("Solving game with {} sides and {} words", game_sides.len(), wordlist_words.len());
    
    // Create the board from the provided sides
    let board = match Board::from_sides(game_sides) {
        Ok(board) => board,
        Err(e) => {
            console_log!("Error creating board: {}", e);
            return vec![format!("Error: {}", e)];
        }
    };
    
    // Create wordlist from provided words
    let dictionary = Dictionary::from_words(wordlist_words);
    
    // Create solver and solve
    let solver = Solver::new(board, wordlist, max_solutions);
    let solutions = solver.solve();
    
    console_log!("Found {} solutions", solutions.len());
    
    // Convert solutions to strings using the Display trait
    solutions.iter().map(|s| s.to_string()).collect()
}