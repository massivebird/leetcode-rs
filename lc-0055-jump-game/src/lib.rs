struct Solution {}

#[allow(dead_code, clippy::needless_pass_by_value)]
impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_0() {
        assert!(Solution::can_jump(vec![2, 3, 1, 1, 4]));
    }

    #[test]
    fn case_1() {
        assert!(!Solution::can_jump(vec![3, 2, 1, 0, 4]));
    }
}
