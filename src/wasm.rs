use wasm_bindgen::prelude::*;
use crate::game::Game;
use crate::wordlist::Wordlist;
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
pub fn solve_game(game_sides: Vec<String>, wordlist_words: Vec<String>) -> Vec<String> {
    console_log!("Solving game with {} sides and {} words", game_sides.len(), wordlist_words.len());
    
    // Create the game from the provided sides
    let game = match Game::from_sides(game_sides) {
        Ok(game) => game,
        Err(e) => {
            console_log!("Error creating game: {}", e);
            return vec![format!("Error: {}", e)];
        }
    };
    
    // Create wordlist from provided words
    let wordlist = Wordlist::from_words(wordlist_words);
    
    // Create solver and solve
    let solver = Solver::new(game, wordlist);
    let solutions = solver.solve();
    
    console_log!("Found {} solutions", solutions.len());
    
    // Convert solutions to strings using the Display trait
    solutions.iter().map(|s| s.to_string()).collect()
}