use std::collections::HashSet;
use std::fmt;
use crate::game::Game;
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

pub struct Solver {
    game: Game,
    possible_words: Vec<String>,
}

impl Solver {
    pub fn new(game: Game, wordlist: Wordlist) -> Self {
        let possible_words = game.possible_words(&wordlist);
        Solver {
            game,
            possible_words,
        }
    }

    pub fn solve(&self) -> Vec<Solution> {
        self.solve_with_max_solutions(100)
    }

    pub fn solve_with_max_solutions(&self, max_solutions: usize) -> Vec<Solution> {
        let mut solutions = Vec::new();
        
        // Try single word solutions first
        for word in &self.possible_words {
            if self.covers_all_letters(&[word]) {
                solutions.push(Solution::new(vec![word.clone()]));
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
        for word1 in &self.possible_words {
            let last_char = word1.chars().last().unwrap();
            
            for word2 in &self.possible_words {
                if word2.chars().next() == Some(last_char) {
                    let word_pair = [word1, word2];
                    if self.covers_all_letters(&word_pair) {
                        two_word_solutions.push(Solution::new(vec![word1.clone(), word2.clone()]));
                        if two_word_solutions.len() >= max_solutions {
                            break;
                        }
                    }
                }
            }
            if two_word_solutions.len() >= max_solutions {
                break;
            }
        }
        
        // Try three word solutions (O(n³) but limited to make it practical)
        let mut three_word_solutions = Vec::new();
        let remaining_slots = max_solutions.saturating_sub(two_word_solutions.len());
        
        'outer: for word1 in &self.possible_words {
            let last_char1 = word1.chars().last().unwrap();
            
            for word2 in &self.possible_words {
                if word2.chars().next() == Some(last_char1) {
                    let last_char2 = word2.chars().last().unwrap();
                    
                    for word3 in &self.possible_words {
                        if word3.chars().next() == Some(last_char2) {
                            let word_trio = [word1, word2, word3];
                            if self.covers_all_letters(&word_trio) {
                                three_word_solutions.push(Solution::new(vec![
                                    word1.clone(), 
                                    word2.clone(), 
                                    word3.clone()
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
    
    fn covers_all_letters(&self, words: &[&String]) -> bool {
        let mut covered_letters = HashSet::new();
        
        for word in words {
            for ch in word.chars() {
                covered_letters.insert(ch);
            }
        }
        
        // Check if we've covered all letters from all sides
        for side in &self.game.sides {
            for ch in side.chars() {
                if !covered_letters.contains(&ch) {
                    return false;
                }
            }
        }
        
        true
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
    fn test_covers_all_letters() {
        let sides = vec![
            "AB".to_string(),
            "CD".to_string(),
            "EF".to_string(), 
            "GH".to_string()
        ];
        let game = Game::from_sides(sides).unwrap();
        let wordlist = Wordlist { 
            words: vec![], 
            word_digraphs: std::collections::HashMap::new(),
            valid_digraphs: std::collections::HashSet::new()
        };
        let solver = Solver::new(game, wordlist);
        
        let word1 = "ABCDEFGH".to_string();
        assert!(solver.covers_all_letters(&[&word1]));
        
        let word2 = "ABCD".to_string();
        assert!(!solver.covers_all_letters(&[&word2]));
        
        let word3 = "EFGH".to_string();
        assert!(solver.covers_all_letters(&[&word2, &word3]));
    }
}