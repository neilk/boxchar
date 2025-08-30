use boxchar::wordlist::{Wordlist, has_adjacent_repeated_letters};

#[test]
fn test_wordlist_struct_from_file() {
    let wordlist = Wordlist::from_path("data/wordlist_test.txt").unwrap();
    
    assert_eq!(wordlist.words.len(), 10);
    assert_eq!(wordlist.words[0], "ADJUNCTIVE");
    assert_eq!(wordlist.words[1], "CONCERNED");
    assert_eq!(wordlist.words[9], "TESTATOR");
    assert!(wordlist.words.contains(&"RANDOM".to_string()));
    assert!(wordlist.words.contains(&"RESET".to_string()));
}

#[test]
fn test_has_adjacent_repeated_letters() {
    assert!(has_adjacent_repeated_letters("PEER"));
    assert!(has_adjacent_repeated_letters("BOOK"));
    assert!(has_adjacent_repeated_letters("HELLO"));
    assert!(has_adjacent_repeated_letters("COFFEE"));
    
    assert!(!has_adjacent_repeated_letters("DOJO"));
    assert!(!has_adjacent_repeated_letters("WORD"));
    assert!(!has_adjacent_repeated_letters("GAME"));
    assert!(!has_adjacent_repeated_letters("WORDS"));
    assert!(!has_adjacent_repeated_letters("A"));
    assert!(!has_adjacent_repeated_letters(""));
}