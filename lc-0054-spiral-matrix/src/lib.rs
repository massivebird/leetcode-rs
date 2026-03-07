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

        let mut turns = 0;

        let capacity = matrix.len() * matrix[0].len();
        let mut ans: Vec<i32> = Vec::with_capacity(capacity);

        let mut pos = true;

        ans.push(matrix[i][j]);

        let mut side_len = matrix[0].len() - 1;

        loop {
            if ans.len() == capacity {
                return ans;
            }

            dbg!((pos, turns % 2 == 0, side_len));
            for _ in 0..side_len {
                match (pos, turns % 2 == 0) {
                    // x-axis
                    (true, true) => j += 1,
                    (false, true) => j -= 1,
                    // y-axis
                    (true, false) => i -= 1,
                    (false, false) => i += 1,
                }

                ans.push(matrix[i][j]);
                dbg!(&ans);
            }

            turns += 1;
            if turns % 2 != 0 {
                pos ^= true;
            }
            // Alternate axis positive directions every two turns.
            if turns != 1 && turns % 2 != 0 {
                side_len -= 1;
            }
        }
    }
}
