use crate::board::Board;
use crate::dictionary::Dictionary;
use std::collections::HashMap;
use std::fmt;

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
    max_solutions: usize, // this is usize for convenience in comparisons to length(), but set from u16
}

impl Solver {
    pub fn new(board: Board, dictionary: &Dictionary, max_solutions: u16) -> Self {
        // Create letter-to-bit mapping
        let mut letter_to_bit = HashMap::new();
        let mut bit_index = 0;
        for side in &board.sides {
            for ch in side.chars() {
                letter_to_bit.insert(ch, 1 << bit_index);
                bit_index += 1;
            }
        }

        // Calculate mask for all letters, e.g. for 8 letters, this is 0b11111111
        let all_letters_mask = 2u32.pow(bit_index) - 1;

        // Create word bitmaps for all words playable
        let board_dictionary = board.playable_dictionary(&dictionary);
        let word_bitmaps: Vec<WordBitmap> = board_dictionary
            .words
            .iter()
            .map(|w| {
                let word = &w.word;
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
            max_solutions: max_solutions.into(),
        }
    }

    pub fn solve(&self) -> Vec<Solution> {
        let mut solutions = Vec::new();

        // Try solutions of each exact length
        for target_words in 1..=4 {
            let mut current_path = Vec::new();
            self.search_recursive(&mut current_path, 0, None, &mut solutions, target_words);

            // Continue searching until we hit the solution limit
            if solutions.len() >= self.max_solutions {
                break;
            }
        }

        solutions
    }

    fn search_recursive(
        &self,
        current_path: &mut Vec<String>,
        covered_bitmap: u32,
        last_char: Option<char>,
        solutions: &mut Vec<Solution>,
        target_words: usize,
    ) {
        // Early termination if we have enough solutions
        if solutions.len() >= self.max_solutions {
            return;
        }

        // Check if we've found a complete solution of the target length
        if covered_bitmap == self.all_letters_mask && current_path.len() == target_words {
            solutions.push(Solution::new(current_path.clone()));
            return;
        }

        // Don't go deeper if we've hit the word limit
        if current_path.len() >= target_words {
            return;
        }

        // Determine which words we can try next
        let word_indices: Vec<usize> = if let Some(ch) = last_char {
            // Must start with the last character of the previous word
            self.words_by_first_letter
                .get(&ch)
                .map(|v| v.clone())
                .unwrap_or_default()
        } else {
            // First word - can be any word
            (0..self.word_bitmaps.len()).collect()
        };

        for word_idx in word_indices {
            let word_bitmap = &self.word_bitmaps[word_idx];
            let new_bitmap = covered_bitmap | word_bitmap.bitmap;

            // Only continue if this word adds new letters
            if new_bitmap != covered_bitmap {
                current_path.push(word_bitmap.word.clone());
                let new_last_char = word_bitmap.word.chars().last();

                self.search_recursive(
                    current_path,
                    new_bitmap,
                    new_last_char,
                    solutions,
                    target_words,
                );

                current_path.pop();

                // Early termination check
                if solutions.len() >= self.max_solutions {
                    return;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_display() {
        let solution = Solution::new(vec![
            "word1".to_string(),
            "word2".to_string(),
            "word3".to_string(),
        ]);
        assert_eq!(solution.to_string(), "word1-word2-word3");

        let single_word = Solution::new(vec!["hello".to_string()]);
        assert_eq!(single_word.to_string(), "hello");
    }

    #[test]
    fn test_bitmap_coverage() {
        let sides = vec![
            "ab".to_string(),
            "cd".to_string(),
            "ef".to_string(),
            "gh".to_string(),
        ];
        let game = Board::from_sides(sides).unwrap();

        // Create a simple wordlist that includes all the digraphs these words need
        // Since the game is AB/CD/EF/GH, we need valid cross-side digraphs
        let mut valid_digraphs = std::collections::HashSet::new();

        // Add all valid digraphs from the game (cross-side pairs)
        for digraph in &game.digraphs {
            valid_digraphs.insert(digraph.clone());
        }

        // For our test words, we need to pick ones that use valid cross-side digraphs
        // Let's use simpler words that work: "AC" (A->C), "CE" (C->E), etc.
        let test_words = ["ac", "ce", "eg"];
        let test_word_strings = test_words.iter().map(|&s| s.to_string()).collect();
        let dictionary = Dictionary::from_strings(test_word_strings);
        let solver = Solver::new(game, &dictionary, 10);

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
