struct Solution;

impl Solution {
    #[allow(dead_code, clippy::needless_pass_by_value)]
    pub fn reverse_words(s: String) -> String {
        s.split_whitespace()
            .rev()
            .fold(String::new(), |acc, s| acc + " " + s)[1..] // Exclude leading space
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_0() {
        assert_eq!(
            Solution::reverse_words("the sky is blue".to_string()),
            "blue is sky the".to_string()
        );
    }

    #[test]
    fn case_1() {
        assert_eq!(
            Solution::reverse_words("  hello world  ".to_string()),
            "world hello".to_string()
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::reverse_words("t ".to_string()), "t".to_string());
    }
}
