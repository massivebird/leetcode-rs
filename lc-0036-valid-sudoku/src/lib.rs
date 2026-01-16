struct Solution;

impl Solution {
    #[allow(dead_code, clippy::needless_pass_by_value)]
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut col_seen = [false; 9];
        let mut row_seen = [false; 9];

        // Check columns and rows.
        for (col_idx, col) in board.iter().enumerate() {
            for row_idx in 0..9 {
                let col_val = col[row_idx].to_digit(10).map(|v| v as usize);
                let row_val = board[row_idx][col_idx].to_digit(10).map(|v| v as usize);

                if let Some(col_val) = col_val {
                    // Return if a duplicate is detected.
                    if col_seen[col_val - 1] {
                        return false;
                    }

                    col_seen[col_val - 1] = true;
                }

                if let Some(row_val) = row_val {
                    if row_seen[row_val - 1] {
                        return false;
                    }

                    row_seen[row_val - 1] = true;
                }
            }

            // Clear memory between columns.
            row_seen = [false; 9];
            col_seen = [false; 9];
        }

        // Check 3x3s.

        let mut tri_seen = row_seen; // Re-use allocation

        for col_idx in [0, 3, 6] {
            for row_idx in [0, 3, 6] {
                // This 3x3 starts at (row_idx, col_idx).

                // Check each space in this 3x3.
                for i in 0..3 {
                    for j in 0..3 {
                        let val = board[row_idx + i][col_idx + j]
                            .to_digit(10)
                            .map(|v| v as usize);

                        if let Some(val) = val {
                            if tri_seen[val - 1] {
                                return false;
                            }

                            tri_seen[val - 1] = true;
                        }
                    }
                }

                // Clear memory between each 3x3.
                tri_seen = [false; 9];
            }
        }

        // Input is compliant to all rules.
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_0() {
        let board = vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];

        assert!(Solution::is_valid_sudoku(board));
    }

    #[test]
    fn case_1() {
        let board = vec![
            vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];

        assert!(!Solution::is_valid_sudoku(board));
    }

    #[test]
    fn case_2() {
        let board = vec![
            vec!['.', '.', '.', '.', '5', '.', '.', '1', '.'],
            vec!['.', '4', '.', '3', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '3', '.', '.', '1'],
            vec!['8', '.', '.', '.', '.', '.', '.', '2', '.'],
            vec!['.', '.', '2', '.', '7', '.', '.', '.', '.'],
            vec!['.', '1', '5', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '2', '.', '.', '.'],
            vec!['.', '2', '.', '9', '.', '.', '.', '.', '.'],
            vec!['.', '.', '4', '.', '.', '.', '.', '.', '.'],
        ];

        assert!(!Solution::is_valid_sudoku(board));
    }
}
