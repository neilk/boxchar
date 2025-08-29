use boxchar::wordlist::{all_digraphs, Wordlist};
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

#[test]
fn test_wordlist_struct_from_file() {
    let wordlist = Wordlist::from_file("data/wordlist_test.txt").unwrap();
    
    assert_eq!(wordlist.words.len(), 10);
    assert_eq!(wordlist.words[0], "ADJUNCTIVE");
    assert_eq!(wordlist.words[1], "CONCERNED");
    assert_eq!(wordlist.words[9], "TESTATOR");
    assert!(wordlist.words.contains(&"RANDOM".to_string()));
    assert!(wordlist.words.contains(&"RESET".to_string()));
}

#[test]
fn test_wordlist_debug_trait() {
    let wordlist = Wordlist::from_file("data/wordlist_test.txt").unwrap();
    let debug_str = format!("{:?}", wordlist);
    assert!(debug_str.contains("Wordlist"));
    assert!(debug_str.contains("ADJUNCTIVE"));
}