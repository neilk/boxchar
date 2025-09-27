use boxchar::board::Board;
use boxchar::wordlist::Wordlist;
use boxchar::solver::Solver;

mod common;
use common::sides_from_strs;

#[test]
fn test_solver_basic() {
    let sides = sides_from_strs(&["YFA", "OTK", "LGW", "RNI"]);
    let game = Board::from_sides(sides).unwrap();
    
    // Create a minimal wordlist for testing
    let words = vec![
        "FORKLIFT".to_string(),
        "TWANGY".to_string(),
        "FILTRATION".to_string(),
        "NAG".to_string(),
        "GAWKILY".to_string(),
    ];
    let wordlist = Wordlist::from_words(words);
    let solver = Solver::new(game, wordlist, 10);
    let solutions = solver.solve();

    assert!(!solutions.is_empty());
    assert!(solutions.len() == 2);
    assert!(solutions.iter().any(|s| s.to_string() == "FORKLIFT-TWANGY"));
    assert!(solutions.iter().any(|s| s.to_string() == "FILTRATION-NAG-GAWKILY"));
}