use boxchar::board::Board;
use boxchar::wordlist::Dictionary;

mod common;
use common::sides_from_strs;

#[test]
fn test_playable_words() {
    let sides = sides_from_strs(&["abc", "def", "gho", "jkl"]);
    let board = Board::from_sides(sides).unwrap();

    let word_strings: Vec<String> = Vec::from([
        "dojo".to_string(),  // possible
        "abode".to_string(), // impossible, "ab" are on the same side
        "joke".to_string(),  // possible
        "egg".to_string()    // impossible, repeated letter
    ]);
    let dictionary = Dictionary::from_strings(word_strings);
    let playable_dictionary = board.playable_dictionary(&dictionary);

    let playable_words: Vec<String> = playable_dictionary.words.iter().map(|w| w.word.clone()).collect();
    assert!(playable_words.contains(&"dojo".to_string()));
    assert!(!playable_words.contains(&"abode".to_string()));
    assert!(playable_words.contains(&"joke".to_string()));
    assert!(!playable_words.contains(&"egg".to_string()));
}
