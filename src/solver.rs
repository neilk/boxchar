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
    wordlist: Wordlist,
    possible_words: Vec<String>,
}

impl Solver {
    pub fn new(game: Game, wordlist: Wordlist) -> Self {
        let possible_words = game.possible_words(&wordlist);
        Solver {
            game,
            wordlist,
            possible_words,
        }
    }

    pub fn solve(&self) -> Vec<Solution> {
        let mut solutions = Vec::new();
        
        // Try single word solutions first
        for word in &self.possible_words {
            if self.covers_all_letters(&[word]) {
                solutions.push(Solution::new(vec![word.clone()]));
            }
        }
        
        // If we found single word solutions, return them
        if !solutions.is_empty() {
            return solutions;
        }
        
        // Try two word solutions (O(n²))
        for word1 in &self.possible_words {
            let last_char = word1.chars().last().unwrap();
            
            for word2 in &self.possible_words {
                if word2.chars().next() == Some(last_char) {
                    let word_pair = [word1, word2];
                    if self.covers_all_letters(&word_pair) {
                        solutions.push(Solution::new(vec![word1.clone(), word2.clone()]));
                    }
                }
            }
        }
        
        // If we found two word solutions, return them
        if !solutions.is_empty() {
            return solutions;
        }
        
        // Try three word solutions (O(n³) but limited to make it practical)
        for word1 in &self.possible_words {
            let last_char1 = word1.chars().last().unwrap();
            
            for word2 in &self.possible_words {
                if word2.chars().next() == Some(last_char1) {
                    let last_char2 = word2.chars().last().unwrap();
                    
                    for word3 in &self.possible_words {
                        if word3.chars().next() == Some(last_char2) {
                            let word_trio = [word1, word2, word3];
                            if self.covers_all_letters(&word_trio) {
                                solutions.push(Solution::new(vec![
                                    word1.clone(), 
                                    word2.clone(), 
                                    word3.clone()
                                ]));
                            }
                        }
                    }
                }
            }
        }
        
        // Sort solutions by score (fewer words is better)
        solutions.sort_by_key(|s| s.score);
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