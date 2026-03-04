//! Given an array nums of distinct integers, return all the possible permutations.
//! You can return the answer in any order.
//!
//! MB: my reliable backtracking approach!

struct Solution;

#[allow(dead_code)]
impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut acc: Vec<Vec<i32>> = Vec::new();

        Self::rec(&mut Vec::new(), &candidates, target, &mut acc);

        acc
    }

    fn rec(this: &mut Vec<i32>, candidates: &[i32], target: i32, acc: &mut Vec<Vec<i32>>) {
        let sum: i32 = this.iter().sum();

        if sum > target {
            return;
        } else if sum == target {
            if !acc.contains(this) {
                acc.push(this.clone());
            }

            return;
        }

        // Ignore candidates that precede the one last used.
        for c in candidates.iter().skip(
            candidates
                .iter()
                .position(|o| this.last().is_some_and(|t| o == t))
                .unwrap_or_default(),
        ) {
            this.push(*c);

            Self::rec(this, candidates, target, acc);

            this.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_0() {
        let nums = [2, 3, 6, 7];
        let target = 7;
        let ans = [vec![2, 2, 3], vec![7]];

        assert_eq!(Solution::combination_sum(nums.into(), target), ans);
    }

    #[test]
    fn case_1() {
        let nums = [7, 3, 2];
        let target = 18;
        let ans = [
            vec![7, 7, 2, 2],
            vec![7, 3, 3, 3, 2],
            vec![7, 3, 2, 2, 2, 2],
            vec![3, 3, 3, 3, 3, 3],
            vec![3, 3, 3, 3, 2, 2, 2],
            vec![3, 3, 2, 2, 2, 2, 2, 2],
            vec![2, 2, 2, 2, 2, 2, 2, 2, 2],
        ];

        assert_eq!(Solution::combination_sum(nums.into(), target), ans);
    }
}
