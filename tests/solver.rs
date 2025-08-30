use boxchar::game::Game;
use boxchar::wordlist::{extract_digraphs,Wordlist};
use boxchar::solver::Solver;

mod common;
use common::sides_from_strs;

#[test]
fn test_solver_basic() {
    let sides = sides_from_strs(&["YFA", "OTK", "LGW", "RNI"]);
    let game = Game::from_sides(sides).unwrap();
    
    // Create a minimal wordlist for testing
    let words = vec![
        "FORKLIFT".to_string(),
        "TWANGY".to_string(),
    ];
    let mut word_digraphs = std::collections::HashMap::new();
    for word in &words {
        let digraphs = extract_digraphs(word);
        word_digraphs.insert(word.clone(), digraphs);
    }
    let wordlist = Wordlist { words, word_digraphs };
    
    let solver = Solver::new(game, wordlist);
    let solutions = solver.solve();
    
    assert!(!solutions.is_empty());
}