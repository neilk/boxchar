TODO:

- the "algorithm X" approach has many issues
- it's returning the wrong type - it should be Vec<Vec<T>>
- we probalby don't want a generic T at all, because to solve this correctly we need to tie words together.
 We should be reducing the matrix with the Algorithm X criteria, but then also, we must only consider words 
 that connected with the previous word. Perhaps we can pass down the subset selected previously, and on None 
 then all are allowed, so we just iterate through rows (faster to have a stable mapping of initial char -> 
    subset and then use Set intersection??)
- The entire MatrixRow concept seems to preclude using ndarray to do this sort of thing

  fn solve(matrix: ArrayView2<bool>, row_mapping: &[usize]) -> Option<Vec<usize>> {
      // ... find best column and row ...

      for (local_row_idx, &original_row_idx) in row_mapping.iter().enumerate() {
          if !matrix[[local_row_idx, col_idx]] {
              continue;
          }

          // Create new mapping for remaining rows
          let new_mapping: Vec<usize> = row_mapping.iter()
              .enumerate()
              .filter(|(i, _)| *i != local_row_idx)
              .map(|(_, &orig_idx)| orig_idx)
              .collect();

          let reduced_matrix = matrix.select(Axis(0), &remaining_rows)
                                    .select(Axis(1), &remaining_cols);

          if let Some(mut solution) = solve(reduced_matrix.view(), &new_mapping) {
              solution.insert(0, original_row_idx); // Use original index
              return Some(solution);
          }
      }
  }
