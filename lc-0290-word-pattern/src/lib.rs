struct Solution;

#[allow(unused, clippy::needless_pass_by_value)]
impl Solution {
    pub fn word_pattern(pattern: String, s: String) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_0() {
        let pattern = "abba".to_string();
        let s = "dog cat cat dog".to_string();

        assert!(Solution::word_pattern(pattern, s));
    }

    #[test]
    fn case_1() {
        let pattern = "abba".to_string();
        let s = "dog cat cat fish".to_string();

        assert!(!Solution::word_pattern(pattern, s));
    }

    #[test]
    fn case_2() {
        let pattern = "aaaa".to_string();
        let s = "dog cat cat dog".to_string();

        assert!(!Solution::word_pattern(pattern, s));
    }
}
