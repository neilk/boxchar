use boxchar::board::Board;
use boxchar::solver::Solver;
use boxchar::dictionary::Dictionary;

mod common;
use common::sides_from_strs;

#[test]
fn test_solver_basic() {
    let sides = sides_from_strs(&["yfa", "otk", "lgw", "rni"]);
    let game = Board::from_sides(sides).unwrap();

    // Create a minimal wordlist for testing
    let words = vec![
        "forklift".to_string(),
        "twangy".to_string(),
        "filtration".to_string(),
        "nag".to_string(),
        "gawkily".to_string(),
    ];
    let wordlist = Dictionary::from_strings(words);
    let solver = Solver::new(game, &wordlist, 10);
    let solutions = solver.solve();

    assert!(!solutions.is_empty());
    assert!(solutions.len() == 2);
    // Solutions now include scores in format "words:score"
    assert!(solutions.iter().any(|s| s.to_string().starts_with("forklift-twangy:")));
    assert!(solutions
        .iter()
        .any(|s| s.to_string().starts_with("filtration-nag-gawkily:")));
}
