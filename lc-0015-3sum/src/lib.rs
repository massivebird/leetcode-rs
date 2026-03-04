struct Solution;

impl Solution {
    #[allow(dead_code, clippy::needless_pass_by_value)]
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        use std::collections::HashSet;

        let mut nums = nums;
        nums.sort_unstable();

        let mut ans: HashSet<Vec<i32>> = HashSet::new();

        for target_idx in 0..nums.len() {
            Self::two_sum(&nums, target_idx, &mut ans);
        }

        ans.into_iter().collect()
    }

    fn two_sum(nums: &[i32], target_idx: usize, ans: &mut std::collections::HashSet<Vec<i32>>) {
        let target_val = nums[target_idx];

        for (i, val1) in nums
            .iter()
            .enumerate()
            .take(nums.len() - 1)
            .filter(|(i, _)| *i != target_idx)
        {
            for (_j, val2) in nums
                .iter()
                .enumerate()
                .skip(i + 1)
                .filter(|(j, _)| *j != target_idx)
            {
                if target_val + val1 + val2 == 0 {
                    let mut vec = vec![*val1, target_val, *val2];
                    vec.sort_unstable();
                    ans.insert(vec);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_0() {
        let nums = [-1, 0, 1, 2, -1, -4];
        let ans = [vec![-1, -1, 2], vec![-1, 0, 1]];

        assert_eq!(Solution::three_sum(nums.into()), ans);
    }

    #[test]
    fn case_1() {
        let nums = [-100, -70, -60, 110, 120, 130, 160];
        let ans = [vec![-100, -60, 160], vec![-70, -60, 130]];

        assert_eq!(Solution::three_sum(nums.into()), ans);
    }
}
