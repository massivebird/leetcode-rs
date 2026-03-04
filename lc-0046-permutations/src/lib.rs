//! Given an array nums of distinct integers, return all the possible permutations.
//! You can return the answer in any order.
//!
//! MB: my reliable backtracking approach!

struct Solution;

#[allow(dead_code)]
impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut acc: Vec<Vec<i32>> = Vec::new();

        Self::rec(Vec::new(), &nums, &mut acc);

        acc
    }

    fn rec(this: Vec<i32>, rem: &[i32], acc: &mut Vec<Vec<i32>>) {
        // Used up all the available numbers.
        // Push the completed permutation!
        if rem.is_empty() {
            acc.push(this);
            return;
        }

        for r in rem {
            Self::rec(
                [this.clone(), [*r].into()].concat(),
                &rem.iter().copied().filter(|v| v != r).collect::<Vec<i32>>(),
                acc,
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_0() {
        let nums = [1, 2, 3];
        let ans = [
            [1, 2, 3],
            [1, 3, 2],
            [2, 1, 3],
            [2, 3, 1],
            [3, 1, 2],
            [3, 2, 1],
        ];

        assert_eq!(Solution::permute(nums.into()), ans);
    }
}
