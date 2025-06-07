struct Solution;

#[allow(unused, clippy::needless_pass_by_value)]
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut idx = 0;
        let mut ans = 0;

        while idx < s.len() {

        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_0() {
        let s = "abcabcbb".to_string();
        let ans = 3;

        assert_eq!(Solution::length_of_longest_substring(s), ans);
    }

    #[test]
    fn case_1() {
        let s = "bbbbb".to_string();
        let ans = 1;

        assert_eq!(Solution::length_of_longest_substring(s), ans);
    }

    #[test]
    fn case_2() {
        let s = "pwwkew".to_string();
        let ans = 3;

        assert_eq!(Solution::length_of_longest_substring(s), ans);
    }
}
