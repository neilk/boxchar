use std::collections::HashMap;
use std::fmt;
use crate::board::Board;
use crate::wordlist::Wordlist;

#[derive(Debug, Clone)]
pub struct Solution {
    pub words: Vec<String>,
    pub score: usize,
}

impl Solution {
    pub fn new(words: Vec<String>) -> Self {
        let score = words.len();
        Solution { words, score }
    }
}

impl fmt::Display for Solution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.words.join("-"))
    }
}

struct WordBitmap {
    word: String,
    bitmap: u32,
}

pub struct Solver {
    word_bitmaps: Vec<WordBitmap>,
    words_by_first_letter: HashMap<char, Vec<usize>>,
    all_letters_mask: u32,
}

impl Solver {
    pub fn new(game: Board, wordlist: Wordlist) -> Self {
        let possible_words = game.possible_words(&wordlist);

        // Create letter-to-bit mapping
        let mut letter_to_bit = HashMap::new();
        let mut bit_index = 0;
        for side in &game.sides {
            for ch in side.chars() {
                letter_to_bit.insert(ch, 1 << bit_index);
                bit_index += 1;
            }
        }

        // Calculate mask for all letters, e.g. for 8 letters, this is 0b11111111
        let all_letters_mask = 2u32.pow(bit_index) - 1;

        // Create word bitmaps
        let word_bitmaps: Vec<WordBitmap> = possible_words
            .iter()
            .map(|word| {
                let bitmap = word.chars().fold(0, |acc, ch| {
                    acc | letter_to_bit.get(&ch).copied().unwrap_or(0)
                });
                WordBitmap {
                    word: word.clone(),
                    bitmap,
                }
            })
            .collect();

        // Index words by first letter
        let mut words_by_first_letter: HashMap<char, Vec<usize>> = HashMap::new();
        for (i, word_bitmap) in word_bitmaps.iter().enumerate() {
            if let Some(first_char) = word_bitmap.word.chars().next() {
                words_by_first_letter.entry(first_char).or_default().push(i);
            }
        }

        Solver {
            word_bitmaps,
            words_by_first_letter,
            all_letters_mask,
        }
    }

    pub fn solve(&self) -> Vec<Solution> {
        self.solve_with_max_solutions(10000)
    }

    pub fn solve_with_max_solutions(&self, max_solutions: usize) -> Vec<Solution> {
        let mut solutions = Vec::new();
        
        // Try single word solutions first
        for word_bitmap in &self.word_bitmaps {
            if word_bitmap.bitmap == self.all_letters_mask {
                solutions.push(Solution::new(vec![word_bitmap.word.clone()]));
                if solutions.len() >= max_solutions {
                    return solutions;
                }
            }
        }
        
        // If we found single word solutions, return them
        if !solutions.is_empty() {
            return solutions;
        }
        
        // Try two word solutions (O(n²))
        let mut two_word_solutions = Vec::new();
        for word1_bitmap in &self.word_bitmaps {
            let last_char = word1_bitmap.word.chars().last().unwrap();

            if let Some(word2_indices) = self.words_by_first_letter.get(&last_char) {
                for &word2_idx in word2_indices {
                    let word2_bitmap = &self.word_bitmaps[word2_idx];
                    let combined_bitmap = word1_bitmap.bitmap | word2_bitmap.bitmap;

                    if combined_bitmap == self.all_letters_mask {
                        two_word_solutions.push(Solution::new(vec![
                            word1_bitmap.word.clone(),
                            word2_bitmap.word.clone()
                        ]));
                        if two_word_solutions.len() >= max_solutions {
                            break;
                        }
                    }
                }
                if two_word_solutions.len() >= max_solutions {
                    break;
                }
            }
        }
        
        // Try three word solutions (O(n³) but limited to make it practical)
        let mut three_word_solutions = Vec::new();
        let remaining_slots = max_solutions.saturating_sub(two_word_solutions.len());

        'outer: for word1_bitmap in &self.word_bitmaps {
            let last_char1 = word1_bitmap.word.chars().last().unwrap();

            if let Some(word2_indices) = self.words_by_first_letter.get(&last_char1) {
                for &word2_idx in word2_indices {
                    let word2_bitmap = &self.word_bitmaps[word2_idx];
                    let last_char2 = word2_bitmap.word.chars().last().unwrap();
                    let combined_bitmap_12 = word1_bitmap.bitmap | word2_bitmap.bitmap;

                    if let Some(word3_indices) = self.words_by_first_letter.get(&last_char2) {
                        for &word3_idx in word3_indices {
                            let word3_bitmap = &self.word_bitmaps[word3_idx];
                            let combined_bitmap = combined_bitmap_12 | word3_bitmap.bitmap;

                            if combined_bitmap == self.all_letters_mask {
                                three_word_solutions.push(Solution::new(vec![
                                    word1_bitmap.word.clone(),
                                    word2_bitmap.word.clone(),
                                    word3_bitmap.word.clone()
                                ]));
                                if three_word_solutions.len() >= remaining_slots {
                                    break 'outer;
                                }
                            }
                        }
                    }
                }
            }
        }
        
        // Combine solutions: two-word first, then three-word
        solutions.extend(two_word_solutions);
        solutions.extend(three_word_solutions);
        
        solutions
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_display() {
        let solution = Solution::new(vec!["WORD1".to_string(), "WORD2".to_string(), "WORD3".to_string()]);
        assert_eq!(solution.to_string(), "WORD1-WORD2-WORD3");
        
        let single_word = Solution::new(vec!["HELLO".to_string()]);
        assert_eq!(single_word.to_string(), "HELLO");
    }

    #[test]
    fn test_bitmap_coverage() {
        let sides = vec![
            "AB".to_string(),
            "CD".to_string(),
            "EF".to_string(),
            "GH".to_string()
        ];
        let game = Board::from_sides(sides).unwrap();

        // Create a simple wordlist that includes all the digraphs these words need
        // Since the game is AB/CD/EF/GH, we need valid cross-side digraphs
        let mut word_digraphs = std::collections::HashMap::new();
        let mut valid_digraphs = std::collections::HashSet::new();

        // Add all valid digraphs from the game (cross-side pairs)
        for digraph in &game.valid_digraphs {
            valid_digraphs.insert(digraph.clone());
        }

        // For our test words, we need to pick ones that use valid cross-side digraphs
        // Let's use simpler words that work: "AC" (A->C), "CE" (C->E), etc.
        let test_words = ["AC", "CE", "EG"];
        for word in test_words {
            let mut digraphs = std::collections::HashSet::new();
            let chars: Vec<char> = word.chars().collect();
            for i in 0..chars.len()-1 {
                let digraph = format!("{}{}", chars[i], chars[i+1]);
                digraphs.insert(digraph);
            }
            word_digraphs.insert(word.to_string(), digraphs);
        }

        let wordlist = Wordlist {
            words: test_words.iter().map(|&s| s.to_string()).collect(),
            word_digraphs,
            valid_digraphs
        };
        let solver = Solver::new(game, wordlist);

        // Test that all letters bitmap is correctly calculated
        assert_eq!(solver.all_letters_mask, 0b11111111); // 8 bits for 8 letters

        // Test that word bitmaps are correctly calculated
        if let Some(word_ac) = solver.word_bitmaps.iter().find(|wb| wb.word == "AC") {
            // A=bit0, C=bit2, so AC should be 0b00000101
            assert_eq!(word_ac.bitmap, 0b00000101);
        }

        if let Some(word_ce) = solver.word_bitmaps.iter().find(|wb| wb.word == "CE") {
            // C=bit2, E=bit4, so CE should be 0b00010100
            assert_eq!(word_ce.bitmap, 0b00010100);
        }

        if let Some(word_eg) = solver.word_bitmaps.iter().find(|wb| wb.word == "EG") {
            // E=bit4, G=bit6, so EG should be 0b01010000
            assert_eq!(word_eg.bitmap, 0b01010000);
        }

        // Test that basic bitmap operations work
        assert!(solver.word_bitmaps.len() > 0);
    }
}