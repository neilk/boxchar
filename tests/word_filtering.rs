use boxchar::board::Board;
use boxchar::wordlist::Wordlist;

mod common;
use common::sides_from_strs;

#[test]
fn test_extract_digraphs_simple() {
    use std::collections::HashSet;
    use boxchar::wordlist::extract_digraphs;
    
    let expected_digraphs: HashSet<String> = ["PI", "IR", "RA", "AT", "TE"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    
    let actual_digraphs = extract_digraphs("PIRATE");
    assert_eq!(actual_digraphs, expected_digraphs);
}

#[test]
fn test_possible_words() {
    let sides = sides_from_strs(&["ABC", "DEF", "GHI", "JKL"]);
    let game = Board::from_sides(sides).unwrap();
    
    let words: Vec<String> = ["ADGJ", "ABCD", "XYZ"].iter().map(|s| s.to_string()).collect();
    let wordlist = Wordlist::from_words(words);
    let possible_words = game.possible_words(&wordlist);
    
    assert!(possible_words.contains(&"ADGJ".to_string()));
    assert!(!possible_words.contains(&"ABCD".to_string()));
    assert!(!possible_words.contains(&"XYZ".to_string()));
}

#[test]
fn test_possible_words_wordlist_file() {
    let sides = sides_from_strs(&["RNY", "ADM", "IUX", "TOZ"]);
    let game = Board::from_sides(sides).unwrap();
    let wordlist = Wordlist::from_path("data/wordlist_test.txt").unwrap();
    
    let possible_words = game.possible_words(&wordlist);
    
    assert!(possible_words.contains(&"RANDOM".to_string()));
    assert!(possible_words.contains(&"RAINOUT".to_string()));
}