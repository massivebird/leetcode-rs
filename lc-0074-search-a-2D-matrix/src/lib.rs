struct Solution;

#[allow(dead_code)]
impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        use std::cmp::Ordering;

        let width = matrix[0].len();

        let mut l: usize = 0;
        let mut r: usize = (matrix.len() * width) - 1;

        // Short-circuit if this element is on the matrix bounds.
        if target == matrix[0][0] || target == matrix[matrix.len() - 1][width - 1] {
            return true;
        }

        // Short-circuit if this element does not fit within the bounds
        // of this array.
        if target < matrix[0][0] || target > matrix[matrix.len() - 1][width - 1] {
            return false;
        }

        let mut i;

        loop {
            i = l.midpoint(r);
            // dbg!((l, r, i));

            let test = matrix[i / width][i % width];

            match test.cmp(&target) {
                Ordering::Equal => return true,

                // Search has completed without finding the target.
                // Return the index where it would be inserted.
                Ordering::Less if l == i => return false,
                Ordering::Greater if r == i => return false,

                // More searching required!
                Ordering::Less => l = i,
                Ordering::Greater => r = i,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_0() {
        let matrix = vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]];
        let target = 3;

        assert!(Solution::search_matrix(matrix, target));
    }

    #[test]
    fn case_1() {
        let matrix = vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]];
        let target = 13;

        assert!(!Solution::search_matrix(matrix, target));
    }

    #[test]
    fn case_2() {
        let nums = vec![vec![1, 3]];
        let target = 3;

        assert!(Solution::search_matrix(nums, target));
    }
}
