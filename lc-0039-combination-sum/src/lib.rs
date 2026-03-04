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

        Self::rec(Vec::new(), &candidates, target, &mut acc);

        acc
    }

    fn rec(this: Vec<i32>, candidates: &[i32], target: i32, acc: &mut Vec<Vec<i32>>) {
        let sum: i32 = this.iter().sum();

        if sum > target {
            return;
        } else if sum == target {
            acc.push(this);
            return;
        }

        for r in candidates {
            Self::rec(
                [this.clone(), [*r].into()].concat(),
                candidates,
                target,
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
        let nums = [2, 3, 6, 7];
        let target = 7;
        let ans = [vec![2, 2, 3], vec![7]];

        assert_eq!(Solution::combination_sum(nums.into(), target), ans);
    }
}
