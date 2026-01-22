//! Given two strings `needle` and `haystack`, return the index of the first
//! occurrence of `needle` in `haystack`, or `-1` if `needle` is not part
//! of `haystack`.

struct Solution;

impl Solution {
    #[allow(dead_code, clippy::needless_pass_by_value)]
    pub fn str_str(haystack: String, needle: String) -> i32 {
        haystack
            .find(&needle)
            .map_or(-1, |idx| i32::try_from(idx).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_0() {
        assert_eq!(
            Solution::str_str("sadbutsad".to_string(), "sad".to_string()),
            0
        );
    }

    #[test]
    fn case_1() {
        assert_eq!(
            Solution::str_str("leetcode".to_string(), "leeto".to_string()),
            -1
        );
    }
}
