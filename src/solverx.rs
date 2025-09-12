use bitvec::prelude::*;
use ndarray::{Array1, Array2, Axis};
// use log::{debug, info};
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::Display;
use std::hash::Hash;

// Minimum number of uncovered letters a word must have to remain interesting
const MIN_UNCOVERED_COUNT: usize = 2;



#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord,Hash)]
pub struct Letter(u8); // 0-25 for A-Z

impl Letter {
    fn new(c: char) -> Option<Self> {
        if c.is_ascii_alphabetic() {
            Some(Letter((c.to_ascii_uppercase() as u8) - b'A'))
        } else {
            None
        }
    }

    fn to_char(self) -> char {
        (self.0 + b'A') as char
    }
}

#[derive(Debug, Clone, Eq, PartialOrd, Ord, PartialEq, Hash)]
pub struct Word(Vec<Letter>);
impl Word {
    fn from_str(s: &str) -> Option<Self> {
        let mut letters = Vec::new();
        for c in s.chars() {
            if let Some(letter) = Letter::new(c) {
                letters.push(letter);
            } else {
                return None; // Invalid character
            }
        }
        Some(Word(letters))
    }

    fn to_string(&self) -> String {
        self.0.iter().map(|l| l.to_char()).collect()
    }
}

#[derive(Debug, Clone)]
pub struct Game {
    letters: HashSet<Letter>,
    words: Vec<Word>,
}

impl Game {
    pub fn new(letters: String, words: Vec<String>) -> Self {
        let letters_hashset: HashSet<Letter> = letters
            .chars()
            .filter_map(|c| Letter::new(c))
            .collect();

        // Convert words to Vec<Vec<Letter>>
        let words_vec_words: Vec<Word> = words.iter()
            .filter(|word| {
                // Filter out words that contain letters not in the universe
                word.chars().all(|c| {
                    if let Some(letter) = Letter::new(c) {
                        letters_hashset.contains(&letter)
                    } else {
                        false
                    }
                })
            })
            .map(|word| Word::from_str(word).unwrap()) // Safe unwrap due to filter above
            .collect();

        Game {
            letters: letters_hashset, 
            words: words_vec_words,
        }
    }


}

pub struct SolverX;
impl SolverX {
    fn create_matrix(letters: &HashSet<Letter>, words: &[Word]) -> Array2<bool> {
        // Assign the letters to arbitrary column indexes. It doesn't matter what, they just have to have one.
        let letters_to_cols: HashMap<&Letter, usize> = letters
            .iter()
            .enumerate()
            .map(|(i, letter)| (letter, i))    
            .collect();

        let row_count = words.len();
        let col_count = letters_to_cols.len();
        let mut matrix = Array2::from_elem((row_count, col_count), false);

        for (row, word) in words.iter().enumerate() {
            for letter in &word.0 {
                if let Some(&col) = letters_to_cols.get(letter) {
                    matrix[[row, col]] = true;
                }
            }
        }
        
        matrix
    }


    pub fn solve(game: &Game) -> Option<Solutions> {        
        if game.words.is_empty() || game.letters.is_empty() {
            return Some(Solutions::empty());
        }
        
        let matrix = Self::create_matrix(&game.letters, &game.words);
        let words_array1 = Array1::from(game.words.clone());

        // Get the solutions, which will be an exotic type. We will convert it to Solutions
        let deques = Self::solve_recursive(&words_array1, &matrix);
        if deques.is_empty() {
            None
        } else {
            let solutions = deques.into_iter()
                .map(|deque| deque.into_iter().collect())
                .map(Solution::new)
                .collect();

            Some(Solutions::new(solutions))
        }
    }

    fn solve_recursive(
        words: &Array1<Word>,
        matrix: &Array2<bool>,
    ) -> Vec<VecDeque<Word>> {
        println!("==== Solving recursively...");
        println!("Matrix shape: ({}, {})", matrix.nrows(), matrix.ncols());
        println!("Words count: {}", words.len());
        println!("Matrix:\n{}", matrix);

        let mut solutions = Vec::new();

        // If matrix is empty, the caller found a solution!
        // We indicate to our callers that we found a solution by returning a Vec with one empty VecDeque
        if matrix.ncols() == 0 {
            solutions.push(VecDeque::new());
            return solutions;
        }
        
        // If any column is empty, no solution exists
        for col in 0..matrix.ncols() {
            let mut has_true = false;
            for row in 0..matrix.nrows() {
                if matrix[[row, col]] {
                    has_true = true;
                    break;
                }
            }
            if !has_true {
                return Vec::new();
            }
        }
                
        // Iterate over all word rows and recurse
        for row in 0..matrix.nrows() {            
            let current_label = words[row].clone();
            
            // Given the current word we are considering, 
            // reduce the matrix of remaining words accordingly for this recursive path. 
            let (reduced_words, reduced_matrix) = Self::reduce_to_interesting(words, matrix, row);

            let tail_solutions = Self::solve_recursive(
                &reduced_words,
                &reduced_matrix,
            );
            
            // Prepend current label to each tail solution
            for mut tail in tail_solutions {
                tail.push_front(current_label.clone());
                solutions.push(tail);
            }
        }
        
        solutions
    }

    /// As we recurse down a particular path, we can reduce the matrix to 
    /// horizontally: remove letters we already have covered
    /// vertically: remove words that don't seem to add much to letters we're going to cover.
    /// 
    /// TODO, IDEA: Knuth's Algorithm X may not be that appropriate since we aren't completely eliminating many words. 
    /// Alg X works by eliminating any subset which has an element that we already have. The "path" downwards through 
    /// all possibilities is then represented by the cut-down matrix. 
    /// 
    /// BUT, the only word we truly eliminate is the one we just picked. Any other word _could_ have a role in a Letter Boxed 
    /// solution, including words which don't add _any_ new letters, because maybe it's just bridging to a word we need later.
    /// 
    /// Since in Letter Boxed we only have 12 letters, we could easily represent a row as a bitmask u16,
    /// And then we simply pass downwards a Vec of which "rows" to consider, which will also handle Letter Boxed's need
    /// to connect words together. i.e. we eliminate some "rows" due to them not being helpful, then we eliminate even more 
    /// due to them not being connectable to the previous word, pass the list down and continue. Then we can actually keep the 
    /// words and matrix completely static!!
    fn reduce_to_interesting(
        words: &Array1<Word>,
        matrix: &Array2<bool>,
        selected_row: usize,
    ) -> (Array1<Word>, Array2<bool>) {
        println!("Reducing matrix based on selected word: {}", words[selected_row].to_string());
        
        let selected_word = &words[selected_row];
        let selected_word_last_char = selected_word.0.last().unwrap();

        // Find letters (cols) that are covered by the selected row
        let mut covered_mask = bitvec![0; matrix.ncols()];
        for col in 0..matrix.ncols() {
            if matrix[[selected_row, col]] {
                covered_mask.set(col, true);
            }
        }

        // Find (words) that are mostly or entirely covered by the existing row
        // The selected row should have zero unconvered letters. So we do not need to special case it. I think.
        let mut excluded_rows = Vec::new();
        for row in 0..matrix.nrows() {
            let uncovered_count = matrix.row(row)
                .iter()
                .enumerate()
                .filter(|&(col, &is_true)| !covered_mask[col] && is_true)
                .count();

            if uncovered_count < MIN_UNCOVERED_COUNT {
                println!("Excluding word {} due to too few uncovered letters", words[row].to_string());
                excluded_rows.push(row);
            } else {
                // If the word does not start with the last letter of the selected word, exclude it
                if let Some(first_letter) = words[row].0.first() {
                    if first_letter != selected_word_last_char {
                        println!("Excluding word {} due to not connecting", words[row].to_string());
                        excluded_rows.push(row);
                    }
                }
            }   
        }
        
        // Create indices for remaining rows and columns
        let remaining_rows: Vec<_> = (0..matrix.nrows())
            .filter(|&row| !excluded_rows.contains(&row))
            .collect();
        let remaining_cols: Vec<_> = (0..matrix.ncols())
            .filter(|&col| !covered_mask[col])
            .collect();
        
        // Use select to create new arrays for labels and the matrix. 
        // The .select method avoids copying!
        let new_words = words.select(Axis(0), &remaining_rows);
        let new_matrix = matrix
            .select(Axis(0), &remaining_rows)
            .select(Axis(1), &remaining_cols);

        (new_words, new_matrix)
    }

}




// A solution containing a single combination of selected words. Glorified word vector
#[derive(Debug, Clone, PartialEq)]
pub struct Solution (pub Vec<Word>); 
impl Solution {
    pub fn new(solution: Vec<Word>) -> Self {
        Self(solution)
    }
}

impl Display for Solution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let solution_string = self.0
            .iter()
            .map(|item| item.to_string())
            .collect::<Vec<_>>()
            .join("-");
        write!(f, "{}", solution_string)
    }
}

/// Solutions containing multiple possible combinations of selected words. Glorified vector of word vectors
#[derive(Debug, Clone, PartialEq)]
pub struct Solutions(pub Vec<Solution>);

impl Solutions {
    pub fn new(solutions: Vec<Solution>) -> Self {
        Self(solutions)
    }

    // Convenience method to create an empty solution set
    pub fn empty() -> Self {
        Self::new(vec![Solution::new(vec![])])
    }
}

impl Display for Solutions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let solution_strings: Vec<String> = self.0
            .iter()
            .map(|solution| {
                solution.to_string()
            })
            .collect();
        
        write!(f, "{}", solution_strings.join("\n"))
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let game = Game::new(
            "ABCDEF".to_string(),
            vec![
                "EBDF".to_string(),
                "ABCD".to_string(),
                "DEF".to_string(),
                "ACE".to_string(),
            ],
        );
        
        let opt_solutions = SolverX::solve(&game);
        assert!(opt_solutions.is_some());
        let solutions = opt_solutions.unwrap();
        println!("Solutions:\n{}", solutions);

        let solutions_vec = solutions.0;        
        
        assert!(!solutions_vec.is_empty());
        assert!(solutions_vec.len() == 2);
        
        // Build expected solutions: ABCD-DEF and ACE-EBDF
        let expected1 = Solution::new(vec![
            Word::from_str("ABCD").unwrap(),
            Word::from_str("DEF").unwrap(),
        ]);
        let expected2 = Solution::new(vec![
            Word::from_str("ACE").unwrap(),
            Word::from_str("EBDF").unwrap(),
        ]); 

        assert!(solutions_vec.contains(&expected1));
        assert!(solutions_vec.contains(&expected2));

    }
/*
    #[test]
    fn test_no_solution() {
        let labels = array![0, 1];
        let matrix = array![
            [true, false],
            [true, false]
        ];
        
        let solution = solve_matrix(labels.view(), matrix.view());
        assert!(solution.is_none());
    }

    #[test]
    fn test_empty_matrix() {
        let labels = Array1::<i32>::zeros(0);
        let matrix = Array2::<bool>::from_elem((0, 0), false);
        
        let solution = solve_matrix(labels.view(), matrix.view());
        assert!(solution.is_some());
        let sol = solution.unwrap().0;
        assert_eq!(sol.len(), 1);
        assert!(sol[0].is_empty());
    }

    #[test]
    fn test_string_labels() {
        let labels_vec = string_labels(&["A", "B", "C"]);
        let labels = Array1::from_vec(labels_vec);
        let matrix = array![
            [true, false, false],
            [false, true, false],
            [false, false, true]
        ];
        
        let solution = solve_matrix(labels.view(), matrix.view());
        assert!(solution.is_some());
        let sol = solution.unwrap().0;
        assert!(!sol.is_empty());
        assert_eq!(sol[0].len(), 3);
    }


    // This test is drawn verbatim from Wikipedia's Algorithm X page
    // https://en.wikipedia.org/wiki/Knuth%27s_Algorithm_X#Example
    #[test]
    fn test_exact_cover_with_string_subsets() {
        // Create universe with elements [1, 2, 3, 4, 5, 6, 7]
        let universe = Game::new(vec![1, 2, 3, 4, 5, 6, 7]);
        
        // Subsets with string labels:
        // A = [1, 4, 7], B = [1, 4], C = [4, 5, 7], 
        // D = [3, 5, 6], E = [2, 3, 6, 7], F = [2, 7]
        let subsets = vec![
            vec![1, 4, 7],    // A
            vec![1, 4],       // B
            vec![4, 5, 7],    // C
            vec![3, 5, 6],    // D
            vec![2, 3, 6, 7], // E
            vec![2, 7],       // F
        ];
        
        let labels = string_labels(&["A", "B", "C", "D", "E", "F"]);
        
        let solution = universe.solve(&labels, &subsets);
        assert!(solution.is_some());
        
        let sol = solution.unwrap().0;
        assert!(!sol.is_empty());
        let mut solution_set = sol[0].clone();
        solution_set.sort();
        
        let expected = string_labels(&["B", "D", "F"]);
        let mut expected_sorted = expected;
        expected_sorted.sort();
        
        assert_eq!(solution_set, expected_sorted);
    }

    #[test]
    fn test_exact_cover_with_many_solutions() {
        // Create universe with elements [1, 2, 3, 4, 5, 6, 7]
        let universe = Game::new(vec![1, 2, 3, 4, 5, 6, 7]);
        
        let subsets = vec![
            vec![1, 2, 3],    // A
            vec![4, 5, 6],    // B
            vec![1, 3, 5],    // C
            vec![2, 4, 6],    // D
            vec![7],         // E
        ];
        
        let labels = string_labels(&["A", "B", "C", "D", "E"]);
        
        let solutions = universe.solve(&labels, &subsets);
        assert!(solutions.is_some());
        let sol = solutions.unwrap().0;
        
        assert!(sol.len() == 2); // There should be two distinct solutions
        // In both cases we get "E" first because it has the rarest element (7)
        let expected0 = string_labels(&["E", "A", "B"]);
        let expected1 = string_labels(&["E", "C", "D"]);

        assert!(sol.contains(&expected0));
        assert!(sol.contains(&expected1));

    }
 */

}