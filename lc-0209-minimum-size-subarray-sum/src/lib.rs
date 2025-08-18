#[allow(dead_code)]
struct Solution;

impl Solution {
    #[allow(clippy::needless_pass_by_value, dead_code)]
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_0() {
        let target = 7;
        let nums = vec![2, 3, 1, 2, 4, 3];
        let ans = 2;

        assert_eq!(Solution::min_sub_array_len(target, nums), ans);
    }

    #[test]
    fn case_1() {
        let target = 4;
        let nums = vec![1, 4, 4];
        let ans = 1;

        assert_eq!(Solution::min_sub_array_len(target, nums), ans);
    }
}
