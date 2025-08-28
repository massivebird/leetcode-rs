struct Solution;

#[allow(unused, clippy::needless_pass_by_value)]
impl Solution {
    pub fn is_valid(s: String) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_0() {
        let s = "()".into();

        assert!(Solution::is_valid(s));
    }

    #[test]
    fn case_1() {
        let s = "(()".into();

        assert!(!Solution::is_valid(s));
    }
}
