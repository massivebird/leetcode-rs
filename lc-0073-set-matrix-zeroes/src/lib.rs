//! <https://leetcode.com/problems/set-matrix-zeroes/submissions/1943419338/?envType=study-plan-v2&envId=top-interview-150>
//!
//! Given an m x n integer matrix matrix, if an element is 0, set its entire row and column to 0's.
//!
//! MB: Pivoted from a simple iterative approach, then to hash set strat, and finally to
//! this O(1) space in-place "flag" strat. This encodes data into the matrix itself,
//! with a caviat.
//!
//! If a zero is found at `matrix[i][j]`, then set the first values in both its row and col
//! to `0`. This indicates that the row/col contains a zero.
//!
//! Input matrix
//! | 1 1 1 |                    1 0 1
//! | 1 0 1 | == mark row/col => 0 0 1
//! | 1 1 1 |                    1 1 1
//!
//! After marking all rows/cols, if the first element in a row is zero, then set all
//! values in that row to zero...
//!
//! BUT one big problem! If we blindly apply this to the marked matrix above:
//!
//! 1 0 1                       0 0 0
//! 0 0 1 == fill rows/cols ==> 0 0 0 <== INCORRECT!
//! 1 1 1                       0 0 1
//!
//! The first rows/cols didn't originally contain zeroes, but they're totally
//! zeroed out now. We have to record _outside the matrix_ if the first row/col
//! contained an original zero.
//!                             Output matrix
//! 1 0 1    fill rows/cols     | 1 0 1 |
//! 0 0 1 == but smartly    ==> | 0 0 0 |
//! 1 1 1                       | 1 0 1 |

#![allow(dead_code)]

mod tests;

struct Solution;

impl Solution {
    #[allow(clippy::ptr_arg)]
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        // "Zero in first i", means there was a zero in the first row.
        // Mutatis mutandis j, col.
        let mut zfi = false;
        let mut zfj = false;

        // Locate and record coords of zeroes.
        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                if matrix[i][j] == 0 {
                    matrix[0][j] = 0;
                    matrix[i][0] = 0;

                    zfi |= i == 0;
                    zfj |= j == 0;
                }
            }
        }

        // Fill rows of zeroes.
        for i in 1..matrix.len() {
            if matrix[i][0] == 0 {
                for j in 0..matrix[0].len() {
                    matrix[i][j] = 0;
                }
            }
        }

        // Fill cols of zeroes.
        for j in 1..matrix[0].len() {
            if matrix[0][j] == 0 {
                for row in &mut *matrix {
                    row[j] = 0;
                }
            }
        }

        if zfi {
            for val in &mut matrix[0] {
                *val = 0;
            }
        }

        if zfj {
            for row in &mut *matrix {
                row[0] = 0;
            }
        }
    }
}
