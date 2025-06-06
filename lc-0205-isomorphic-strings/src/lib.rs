struct Solution;

#[allow(unused, clippy::needless_pass_by_value)]
impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn case_0() {
        let s = "egg".to_string();
        let t = "add".to_string();

        assert!(Solution::is_isomorphic(s, t));
    }

    #[test]
    fn case_1() {
        let s = "foo".to_string();
        let t = "bar".to_string();

        assert!(!Solution::is_isomorphic(s, t));
    }

    #[test]
    fn case_2() {
        let s = "paper".to_string();
        let t = "title".to_string();

        assert!(Solution::is_isomorphic(s, t));
    }
}
