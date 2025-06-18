struct Solution;

#[allow(unused, clippy::needless_pass_by_value)]
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_0() {
        let nums = vec![2, 2, 3, 2];
        let ans = 3;

        assert_eq!(Solution::single_number(nums), ans);
    }

    #[test]
    fn case_1() {
        let nums = vec![0, 1, 0, 1, 0, 1, 99];
        let ans = 99;

        assert_eq!(Solution::single_number(nums), ans);
    }
}
