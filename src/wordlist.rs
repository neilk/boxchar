use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use std::collections::{HashMap, HashSet};

pub fn has_adjacent_repeated_letters(word: &str) -> bool {
    let chars: Vec<char> = word.chars().collect();
    for i in 0..chars.len().saturating_sub(1) {
        if chars[i] == chars[i + 1] {
            return true;
        }
    }
    false
}

pub fn extract_digraphs(word: &str) -> HashSet<String> {
    let chars: Vec<char> = word.chars().collect();
    let mut digraphs = HashSet::new();
    
    for i in 0..chars.len().saturating_sub(1) {
        let digraph = format!("{}{}", chars[i], chars[i + 1]);
        digraphs.insert(digraph);
    }
    
    digraphs
}

#[derive(Debug)]
pub struct Wordlist {
    pub words: Vec<String>,
    pub word_digraphs: HashMap<String, HashSet<String>>,
}

impl Wordlist {
    pub fn from_path<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let words: Vec<String> = reader
            .lines()
            .map_while(Result::ok)
            .filter(|word| !has_adjacent_repeated_letters(word))
            .collect();
        
        let mut word_digraphs = HashMap::new();
        for word in &words {
            let digraphs = extract_digraphs(word);
            word_digraphs.insert(word.clone(), digraphs);
        }
        
        Ok(Wordlist { words, word_digraphs })
    }
}