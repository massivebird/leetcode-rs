// Given a `pattern` and a string `s`, find if `s` follows the same pattern.
//
// Here, "follow" means a full match, such that there is a bijection between a
// letter in `pattern` and a non-empty word in `s`.

struct Solution;

impl Solution {
    #[allow(dead_code, clippy::needless_pass_by_value)]
    pub fn word_pattern(pattern: String, s: String) -> bool {
        use std::collections::HashMap;

        let mut pattern_str: HashMap<char, &str> = HashMap::new();
        let mut str_pattern: HashMap<&str, char> = HashMap::new();

        if pattern.chars().count() != s.split_whitespace().count() {
            return false;
        }

        for (p, s) in pattern.chars().zip(s.split_whitespace()) {
            if pattern_str.insert(p, s).is_some_and(|old| old != s)
                || str_pattern.insert(s, p).is_some_and(|old| old != p)
            {
                return false;
            }
        }

        true
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

    #[test]
    fn case_3() {
        let pattern = "aaa".to_string();
        let s = "aa aa aa aa".to_string();

        assert!(!Solution::word_pattern(pattern, s));
    }
}
