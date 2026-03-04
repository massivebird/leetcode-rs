struct Solution;

impl Solution {
    #[allow(dead_code, clippy::needless_pass_by_value)]
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        todo!()
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
}
