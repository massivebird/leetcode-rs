struct Solution;

#[allow(unused, clippy::needless_pass_by_value)]
impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_0() {
        let a = "11".to_owned();
        let b = "1".to_owned();
        let ans = "100".to_owned();

        assert_eq!(Solution::add_binary(a, b), ans);
    }

    #[test]
    fn case_1() {
        let a = "1010".to_owned();
        let b = "1011".to_owned();
        let ans = "10101".to_owned();

        assert_eq!(Solution::add_binary(a, b), ans);
    }
}
