struct Solution {}

#[allow(dead_code, clippy::needless_pass_by_value)]
impl Solution {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_0() {
        assert_eq!(Solution::h_index(vec![3, 0, 6, 1, 5]), 3);
    }

    #[test]
    fn case_1() {
        assert_eq!(Solution::h_index(vec![1, 3, 1]), 1);
    }
}
