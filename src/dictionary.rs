use std::collections::{HashSet};
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

/**
 * Note that we depend on the wordlist already being filtered to words which are
 * playable in our game.
 */

pub fn extract_digraphs(word: &String) -> HashSet<String> {
    let chars: Vec<char> = word.chars().collect();
    let mut digraphs = HashSet::new();

    for i in 0..chars.len() - 1 {
        let digraph = format!("{}{}", chars[i], chars[i + 1]);
        digraphs.insert(digraph);
    }

    digraphs
}

#[derive(Debug, Clone)]
pub struct Word {
    pub word: String,
    pub frequency: i8,
    pub digraphs: HashSet<String>,
}

#[derive(Debug)]
pub struct Dictionary {
    pub words: Vec<Word>,
    pub digraphs: HashSet<String>,
}

impl Dictionary {
    const DEFAULT_FREQUENCY: i8 = 15;
    pub fn from_words(words: Vec<Word>) -> Self {
        let mut valid_digraphs = HashSet::new();

        for word in &words {
            valid_digraphs.extend(word.digraphs.iter().cloned());
        }

        Dictionary {
            words,
            digraphs: valid_digraphs,
        }
    }

    // This is only used for tests, and so it has a fake frequency
    pub fn from_strings(words: Vec<String>) -> Self {
        let word_frequencies: Vec<Word> = words
            .into_iter()
            .map(|w| {
                let digraphs = extract_digraphs(&w);
                Word {
                    word: w,
                    frequency: Self::DEFAULT_FREQUENCY,
                    digraphs,
                }
            })
            .collect();
        Self::from_words(word_frequencies)
    }

    fn parse_word_line(line: &str) -> Option<Word> {
        let mut parts = line.trim().split_whitespace();
        match (parts.next(), parts.next()) {
            (Some(word_str), Some(frequency_str)) => match frequency_str.parse::<i8>() {
                Ok(frequency) => Some(Word {
                    word: word_str.to_string(),
                    frequency,
                    digraphs: extract_digraphs(&word_str.to_string()),
                }),
                Err(_) => None,
            },
            _ => None,
        }
    }

    pub fn from_text(text: &str) -> Self {
        let words: Vec<Word> = text
            .lines()
            .filter_map(Self::parse_word_line)
            .collect();
        Self::from_words(words)
    }

    pub fn from_path<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let words: Vec<Word> = reader
            .lines()
            .map_while(Result::ok)
            .enumerate()
            .filter_map(|(line_num, s)| {
                Self::parse_word_line(&s).or_else(|| {
                    eprintln!("Invalid format on line {}: {}", line_num + 1, s);
                    None
                })
            })
            .collect();
        Ok(Self::from_words(words))
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_digraphs_simple() {
        let expected_digraphs: HashSet<String> = ["PI", "IR", "RA", "AT", "TE"]
            .iter()
            .map(|s| s.to_string())
            .collect();

        let actual_digraphs = extract_digraphs(&"PIRATE".to_string());
        assert_eq!(actual_digraphs, expected_digraphs);
    }
}

