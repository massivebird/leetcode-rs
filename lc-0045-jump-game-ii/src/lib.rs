struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_0() {
        assert_eq!(Solution::jump(vec![2, 3, 1, 1, 4]), 2);
    }

    #[test]
    fn case_1() {
        assert_eq!(Solution::jump(vec![2, 3, 0, 1, 4]), 2);
    }
}
