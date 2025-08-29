use boxchar::wordlist::all_digraphs;
use std::fs::File;

#[test]
fn test_read_wordlist_test_file() {
    let file = File::open("data/wordlist_test.txt").unwrap();
    let words = all_digraphs(file).unwrap();
    
    assert_eq!(words.len(), 10);
    assert_eq!(words[0], "ADJUNCTIVE");
    assert_eq!(words[1], "CONCERNED");
    assert_eq!(words[9], "TESTATOR");
    assert!(words.contains(&"RANDOM".to_string()));
    assert!(words.contains(&"RESET".to_string()));
}