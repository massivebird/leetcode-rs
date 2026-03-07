//! <https://leetcode.com/problems/spiral-matrix/description/?envType=study-plan-v2&envId=top-interview-150>
//!
//! Given an m x n matrix, return all elements of the matrix in spiral order.
//! (refer to `tests.rs`)
//!
//! MB: all math and logic. Unchecked array indexing. I figured out (a) pattern
//! because I'm a baller. I'm a beast.

mod tests;

struct Solution;

impl Solution {
    #[allow(dead_code, clippy::needless_pass_by_value)]
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        if matrix.is_empty() {
            return Vec::new();
        }

        let mut i = 0;
        let mut j = 0;

        // Number of times traversal has changed direction.
        let mut turns = 0;

        let capacity = matrix.len() * matrix[0].len();
        let mut ans: Vec<i32> = Vec::with_capacity(capacity);

        let mut pos = true;

        // Push the first element.
        // Makes the `for` loop simpler later.
        ans.push(matrix[i][j]);

        let mut side_len_x = matrix[0].len() - 1;
        let mut side_len_y = matrix.len() - 1;

        loop {
            // Return after exhausting all elements.
            if ans.len() == capacity {
                return ans;
            }

            // dbg!((pos, turns % 2 == 0, side_len_x, side_len_y));

            let bound = if turns % 2 == 0 {
                side_len_x
            } else {
                side_len_y
            };

            for _ in 0..bound {
                match (pos, turns % 2 == 0) {
                    // x-axis
                    (true, true) => j += 1,
                    (false, true) => j -= 1,
                    // y-axis
                    (true, false) => i -= 1,
                    (false, false) => i += 1,
                }

                ans.push(matrix[i][j]);
                // dbg!(&ans);
            }

            turns += 1;

            // Switch positive/negative axis direction every two turns.
            if turns % 2 != 0 {
                pos ^= true;
            }

            // EXCEPT FOR THE FIRST TURN, decrement vertical side length.
            if turns != 1 && turns % 2 != 0 {
                side_len_y -= 1;
            }

            // EXCEPT FOR THE FIRST TWO TURNS, decrement horizontal side length.
            if turns >= 3 && turns % 2 != 0 {
                side_len_x -= 1;
            }
        }
    }
}
