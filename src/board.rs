use crate::wordlist::Wordlist;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

#[derive(Debug)]
pub struct Board {
    pub sides: Vec<String>,
    pub valid_digraphs: HashSet<String>,
}

impl Board {
    pub fn from_sides(sides: Vec<String>) -> io::Result<Self> {
        Self::validate_structure(&sides)?;
        Self::validate_content(&sides)?;

        let mut game = Board {
            sides,
            valid_digraphs: HashSet::new(),
        };
        game.compute_valid_digraphs();
        Ok(game)
    }

    pub fn from_path<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let sides: Vec<String> = reader.lines().collect::<io::Result<Vec<String>>>()?;

        Self::from_sides(sides)
    }

    fn validate_structure(sides: &[String]) -> io::Result<()> {
        if sides.len() != 4 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Game must contain exactly 4 sides, found {}", sides.len()),
            ));
        }

        if sides.iter().any(|side| side.is_empty()) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Empty sides are not allowed",
            ));
        }

        let first_len = sides[0].len();
        for (i, side) in sides.iter().enumerate() {
            if side.len() != first_len {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("All sides must have the same length. Side 0 has length {} but side {} has length {}", 
                        first_len, i, side.len())
                ));
            }
        }

        Ok(())
    }

    fn validate_content(sides: &[String]) -> io::Result<()> {
        let mut seen_chars: HashMap<char, usize> = HashMap::new();

        for (side_num, side) in sides.iter().enumerate() {
            for c in side.chars() {
                if !c.is_ascii_uppercase() {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("Invalid character '{}' on side {}. Only uppercase ASCII letters are allowed", 
                            c, side_num)
                    ));
                }

                if let Some(previous_side) = seen_chars.insert(c, side_num) {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!(
                            "Duplicate letter '{}' found on sides {} and {}",
                            c, previous_side, side_num
                        ),
                    ));
                }
            }
        }

        Ok(())
    }

    fn compute_valid_digraphs(&mut self) {
        for (i, side) in self.sides.iter().enumerate() {
            for c1 in side.chars() {
                for (j, other_side) in self.sides.iter().enumerate() {
                    if i != j {
                        for c2 in other_side.chars() {
                            let digraph = format!("{}{}", c1, c2);
                            self.valid_digraphs.insert(digraph);
                        }
                    }
                }
            }
        }
    }

    pub fn possible_words(&self, wordlist: &Wordlist) -> Vec<String> {
        let usable_digraphs: HashSet<&String> = self
            .valid_digraphs
            .intersection(&wordlist.valid_digraphs)
            .collect();

        wordlist
            .words
            .iter()
            .filter(|word| {
                if let Some(required_digraphs) = wordlist.word_digraphs.get(*word) {
                    required_digraphs
                        .iter()
                        .all(|d| usable_digraphs.contains(d))
                } else {
                    false
                }
            })
            .cloned()
            .collect()
    }
}
