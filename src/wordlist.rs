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
    pub valid_digraphs: HashSet<String>,
}

impl Wordlist {
    pub fn from_words(words: Vec<String>) -> Self {
        let mut word_digraphs = HashMap::new();
        let mut valid_digraphs = HashSet::new();

        for word in &words {
            if has_adjacent_repeated_letters(word) {
                continue;
            }
            let digraphs = extract_digraphs(word);
            valid_digraphs.extend(digraphs.iter().cloned());
            word_digraphs.insert(word.clone(), digraphs);
        }

        Wordlist { words, word_digraphs, valid_digraphs }
    }

    pub fn from_path<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let words: Vec<String> = reader
            .lines()
            .map_while(Result::ok)
            .collect();
        Ok(Self::from_words(words))
    }
}