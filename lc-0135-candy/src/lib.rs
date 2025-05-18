struct Solution {}

#[allow(unused, clippy::needless_pass_by_value)]
impl Solution {
    pub fn candy(ratings: Vec<i32>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_0() {
        assert_eq!(Solution::candy(vec![1, 0, 2]), 5);
    }

    #[test]
    fn case_1() {
        assert_eq!(Solution::candy(vec![1, 2, 2]), 4);
    }
}
