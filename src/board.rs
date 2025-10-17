use crate::dictionary::Dictionary;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

const SIDES_DISPLAY: &[&str] = &["top", "right", "bottom", "left"];

#[derive(Debug, Clone)]
pub struct Board {
    pub sides: Vec<String>,
    pub digraphs: HashSet<String>,
}

impl Board {
    pub fn from_sides(sides: Vec<String>) -> io::Result<Self> {
        Self::validate_sides_structure(&sides)?;
        Self::validate_sides_content(&sides)?;

        let digraphs = Self::playable_digraphs(&sides);
        let game = Board { sides, digraphs };

        Ok(game)
    }

    pub fn from_path<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let sides: Vec<String> = reader
            .lines()
            .map_while(Result::ok)
            .map(|s| s.to_lowercase())
            .collect();

        Self::from_sides(sides)
    }

    fn validate_sides_structure(sides: &[String]) -> io::Result<()> {
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
                    format!("All sides must have the same length. The {} side has length {} but the {} side has length {}", 
                        SIDES_DISPLAY[0], first_len, SIDES_DISPLAY[i], side.len())
                ));
            }
        }

        Ok(())
    }

    fn validate_sides_content(sides: &[String]) -> io::Result<()> {
        let mut seen_chars: HashMap<char, usize> = HashMap::new();

        for (side_num, side) in sides.iter().enumerate() {
            for c in side.chars() {
                if !c.is_ascii_lowercase() {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("Invalid character '{}' on the {} side. Only lowercase ASCII letters are allowed", 
                            c, SIDES_DISPLAY[side_num])
                    ));
                }

                if let Some(previous_side) = seen_chars.insert(c, side_num) {
                    let error = if previous_side == side_num {
                        format!("Duplicate letter '{}' found on the {} side", c, SIDES_DISPLAY[side_num])
                    } else {
                        format!(
                            "Duplicate letter '{}' found on the {} side and the {} side",
                            c, SIDES_DISPLAY[previous_side], SIDES_DISPLAY[side_num]
                        )
                    };
                    return Err(io::Error::new(io::ErrorKind::InvalidData, error));
                }
            }
        }

        Ok(())
    }

    fn playable_digraphs(sides: &[String]) -> HashSet<String> {
        let mut digraphs = HashSet::new();
        for (i, side) in sides.iter().enumerate() {
            for c1 in side.chars() {
                for (j, other_side) in sides.iter().enumerate() {
                    if i != j {
                        for c2 in other_side.chars() {
                            let digraph = format!("{}{}", c1, c2);
                            digraphs.insert(digraph);
                        }
                    }
                }
            }
        }
        digraphs
    }

    pub fn playable_dictionary(&self, dictionary: &Dictionary) -> Dictionary {
        // Eliminate any digraphs on this board which are totally impossible in the whole dictionary, e.g. 'vz', 'zq'
        let usable_digraphs: HashSet<&String> =
            self.digraphs.intersection(&dictionary.digraphs).collect();

        // Then cut it down to words which are playable on this board
        let playable_words = dictionary
            .words
            .iter()
            .filter(|word| word.digraphs.iter().all(|d| usable_digraphs.contains(d)))
            .cloned()
            .collect();

        Dictionary::from_words(playable_words)
    }
}
