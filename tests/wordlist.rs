use boxchar::wordlist::Wordlist;

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