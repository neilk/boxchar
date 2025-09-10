use ndarray::{Array1, Array2, ArrayView1, ArrayView2, Axis};
use std::collections::{HashMap, VecDeque};

/// Universe containing elements that need to be covered exactly once
#[derive(Debug, Clone)]
pub struct Universe<T> {
    /// All elements that need to be covered exactly once
    pub elements: Vec<T>,
}

impl<T> Universe<T>
where
    T: Clone + Eq + std::hash::Hash,
{
    /// Create a new Universe with elements
    pub fn new(elements: Vec<T>) -> Self {
        Self { elements }
    }
    
    /// Solve the exact cover problem using Knuth's Algorithm X
    /// 
    /// # Arguments
    /// * `subset_labels` - Labels for each subset
    /// * `subsets` - Collections of elements, each representing a potential subset to select
    /// 
    /// # Returns
    /// * `Some(Solution)` if a solution exists, `None` otherwise
    pub fn solve<L>(&self, subset_labels: &[L], subsets: &[Vec<T>]) -> Option<Solution<L>>
    where
        L: Clone,
    {
        if subset_labels.len() != subsets.len() {
            return None;
        }
        
        if subsets.is_empty() || self.elements.is_empty() {
            return Some(Solution::new(vec![Vec::new()]));
        }
        
        // Create the matrix representation
        let matrix = create_matrix_from_subsets(&self.elements, subsets);
        let labels_array = Array1::from_vec(subset_labels.to_vec());
        
        // Convert to working arrays
        let mut working_labels = labels_array;
        let mut working_matrix = matrix;
        
        let solutions = solve_recursive(&mut working_labels, &mut working_matrix);
        if solutions.is_empty() {
            None
        } else {
            let vec_solutions: Vec<Vec<L>> = solutions.into_iter()
                .map(|deque| deque.into_iter().collect())
                .collect();
            Some(Solution::new(vec_solutions))
        }
    }
}

/// Solutions containing multiple possible combinations of selected subset labels
#[derive(Debug, Clone, PartialEq)]
pub struct Solution<L> {
    pub solutions: Vec<Vec<L>>,
}

impl<L> Solution<L> {
    pub fn new(solutions: Vec<Vec<L>>) -> Self {
        Self { solutions }
    }
}

/// Solve the exact cover problem using Knuth's Algorithm X with raw matrix
/// 
/// # Arguments
/// * `subset_labels` - Labels for each subset (row)
/// * `matrix` - Boolean matrix where rows represent subsets and columns represent constraints
/// 
/// # Returns
/// * `Some(Solution)` if a solution exists, `None` otherwise
pub fn solve_matrix<L: Clone>(
    subset_labels: ArrayView1<L>,
    matrix: ArrayView2<bool>,
) -> Option<Solution<L>> {
    let rows = matrix.nrows();
    let cols = matrix.ncols();
    
    if rows != subset_labels.len() {
        return None;
    }
    
    if rows == 0 || cols == 0 {
        return Some(Solution::new(vec![Vec::new()]));
    }
    
    // Convert to working arrays
    let mut working_labels = subset_labels.to_owned();
    let mut working_matrix = matrix.to_owned();
    
    let solutions = solve_recursive(&mut working_labels, &mut working_matrix);
    if solutions.is_empty() {
        None
    } else {
        let vec_solutions: Vec<Vec<L>> = solutions.into_iter()
            .map(|deque| deque.into_iter().collect())
            .collect();
        Some(Solution::new(vec_solutions))
    }
}

fn solve_recursive<L: Clone>(
    labels: &mut Array1<L>,
    matrix: &mut Array2<bool>,
) -> Vec<VecDeque<L>> {
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
    
    // Choose column with minimum number of 1s (heuristic)
    let chosen_col = choose_column(&matrix);
    
    
    // Try each row that has a 1 in the chosen column
    for row in 0..matrix.nrows() {
        if !matrix[[row, chosen_col]] {
            continue;
        }
        
        let current_label = labels[row].clone();
        
        // Reduce the matrix by selecting this row
        let (mut reduced_labels, mut reduced_matrix) = remove_row_and_covered_columns(labels, matrix, row);

        let tail_solutions = solve_recursive(
            &mut reduced_labels,
            &mut reduced_matrix,
        );
        
        // Prepend current label to each tail solution
        for mut tail in tail_solutions {
            tail.push_front(current_label.clone());
            solutions.push(tail);
        }
    }
    
    solutions
}

/// Choose the column with the minimum number of 1s (Knuth's heuristic)
fn choose_column(matrix: &Array2<bool>) -> usize {
    let mut min_count = matrix.nrows() + 1;
    let mut chosen_col = 0;
    
    for col in 0..matrix.ncols() {
        let count = (0..matrix.nrows())
            .map(|row| if matrix[[row, col]] { 1 } else { 0 })
            .sum::<usize>();
        
        if count < min_count {
            min_count = count;
            chosen_col = col;
        }
    }
    
    chosen_col
}

/// Select a row and reduce the matrix accordingly using ndarray's select method
fn remove_row_and_covered_columns<T: Clone>(
    labels: &Array1<T>,
    matrix: &Array2<bool>,
    selected_row: usize,
) -> (Array1<T>, Array2<bool>) {
    // Find columns that are covered by the selected row
    let mut covered_cols = Vec::new();
    for col in 0..matrix.ncols() {
        if matrix[[selected_row, col]] {
            covered_cols.push(col);
        }
    }

    // Find rows that have at least one column already covered with the selected row
    let mut rows_already_covered = Vec::new();
    for row in 0..matrix.nrows() {
        for &col in &covered_cols {
            if matrix[[row, col]] {
                rows_already_covered.push(row);
                break;
            }
        }
    }
    
    // Create indices for remaining rows and columns
    let remaining_rows: Vec<_> = (0..matrix.nrows())
        .filter(|&row| !rows_already_covered.contains(&row))
        .collect();
    let remaining_cols: Vec<_> = (0..matrix.ncols())
        .filter(|&col| !covered_cols.contains(&col))
        .collect();
    
    // Use select to create new arrays for labels and the matrix. 
    // The .select method avoids copying!
    let new_labels = labels.select(Axis(0), &remaining_rows);
    let new_matrix = matrix
        .select(Axis(0), &remaining_rows)
        .select(Axis(1), &remaining_cols);

    (new_labels, new_matrix)
}

/// Helper function to create a boolean matrix from universe and subsets
/// 
/// # Arguments
/// * `universe` - All elements that need to be covered
/// * `subsets` - Collections of elements, each representing a potential subset to select
/// 
/// # Returns
/// * Boolean matrix where rows represent subsets and columns represent universe elements
pub fn create_matrix_from_subsets<T: Clone + Eq + std::hash::Hash>(
    universe: &[T],
    subsets: &[Vec<T>],
) -> Array2<bool> {
    let universe_map: HashMap<&T, usize> = universe
        .iter()
        .enumerate()
        .map(|(i, elem)| (elem, i))
        .collect();
    
    let mut matrix = Array2::from_elem((subsets.len(), universe.len()), false);
    
    for (row, subset) in subsets.iter().enumerate() {
        for element in subset {
            if let Some(&col) = universe_map.get(element) {
                matrix[[row, col]] = true;
            }
        }
    }
    
    matrix
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::{array, Array2};

    #[test]
    fn test_simple_exact_cover() {
        // Simple exact cover problem from Wikipedia example
        let labels = array![0, 1, 2, 3, 4, 5];
        let matrix = array![
            [true, false, false, true, false, false],
            [true, false, false, true, false, false],
            [false, false, false, true, true, false],
            [false, false, true, false, true, true],
            [false, true, true, false, false, true],
            [false, true, false, false, false, false]
        ];
        
        let solution = solve_matrix(labels.view(), matrix.view());
        assert!(solution.is_some());
        let sol = solution.unwrap();
        // Should find a valid solution
        assert!(!sol.solutions.is_empty());
    }

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
        let sol = solution.unwrap();
        assert_eq!(sol.solutions.len(), 1);
        assert!(sol.solutions[0].is_empty());
    }

    #[test]
    fn test_string_labels() {
        let labels = array!["A".to_string(), "B".to_string(), "C".to_string()];
        let matrix = array![
            [true, false, false],
            [false, true, false],
            [false, false, true]
        ];
        
        let solution = solve_matrix(labels.view(), matrix.view());
        assert!(solution.is_some());
        let sol = solution.unwrap();
        assert!(!sol.solutions.is_empty());
        assert_eq!(sol.solutions[0].len(), 3);
    }


    // This test is drawn verbatim from Wikipedia's Algorithm X page
    // https://en.wikipedia.org/wiki/Knuth%27s_Algorithm_X#Example
    #[test]
    fn test_exact_cover_with_string_subsets() {
        // Create universe with elements [1, 2, 3, 4, 5, 6, 7]
        let universe = Universe::new(vec![1, 2, 3, 4, 5, 6, 7]);
        
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
        
        let labels = vec![
            "A".to_string(),
            "B".to_string(), 
            "C".to_string(),
            "D".to_string(),
            "E".to_string(),
            "F".to_string()
        ];
        
        let solution = universe.solve(&labels, &subsets);
        assert!(solution.is_some());
        
        let sol = solution.unwrap();
        assert!(!sol.solutions.is_empty());
        let mut solution_set = sol.solutions[0].clone();
        solution_set.sort();
        
        let expected = vec!["B".to_string(), "D".to_string(), "F".to_string()];
        let mut expected_sorted = expected;
        expected_sorted.sort();
        
        assert_eq!(solution_set, expected_sorted);
    }

// This test is drawn verbatim from Wikipedia's Algorithm X page
    // https://en.wikipedia.org/wiki/Knuth%27s_Algorithm_X#Example
    #[test]
    fn test_exact_cover_with_many_solutions() {
        // Create universe with elements [1, 2, 3, 4, 5, 6, 7]
        let universe = Universe::new(vec![1, 2, 3, 4, 5, 6, 7]);
        
        // Subsets with string labels:
        // A = [1, 4, 7], B = [1, 4], C = [4, 5, 7], 
        // D = [3, 5, 6], E = [2, 3, 6, 7], F = [2, 7]
        let subsets = vec![
            vec![1, 4, 7],    // A
            vec![1, 4],       // B
            vec![4, 5, 7],    // C
            vec![3, 5, 6],    // D
            vec![1, 2, 3, 6, 7], // E
            vec![2, 7],       // F
        ];
        
        let labels = vec![
            "A".to_string(),
            "B".to_string(), 
            "C".to_string(),
            "D".to_string(),
            "E".to_string(),
            "F".to_string()
        ];
        
        let solution = universe.solve(&labels, &subsets);
        assert!(solution.is_some());
        
        let sol = solution.unwrap();
        assert!(!sol.solutions.is_empty());
        let mut solution_set = sol.solutions[0].clone();
        solution_set.sort();
        
        let expected = vec!["B".to_string(), "D".to_string(), "F".to_string()];
        let mut expected_sorted = expected;
        expected_sorted.sort();
        
        assert_eq!(solution_set, expected_sorted);
    }

    #[test]
    fn test_exact_cover_with_one_subset_with_everything() {
        // Create universe with elements [1, 2, 3, 4, 5, 6, 7]
        let universe = Universe::new(vec![1, 2, 3, 4, 5, 6, 7]);
        
        let subsets = vec![
            vec![1, 2, 3, 4, 5, 6, 7],    // A
            vec![1, 4],       // B
            vec![4, 5, 7],    // C
        ];
        
        let labels = vec![
            "A".to_string(),
            "B".to_string(), 
            "C".to_string(),
        ];
        
        let solution = universe.solve(&labels, &subsets);
        assert!(solution.is_some());
        
        let sol = solution.unwrap();
        assert!(!sol.solutions.is_empty());
        let solution_set = sol.solutions[0].clone();
        
        let expected = vec!["A".to_string()];
        
        assert_eq!(solution_set, expected);
    }

}