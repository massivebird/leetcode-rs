struct Solution;

impl Solution {
    #[allow(dead_code, clippy::needless_pass_by_value)]
    pub fn is_palindrome(s: String) -> bool {
        let s: String = s
            .chars()
            .filter(|c| c.is_alphanumeric())
            .map(|c| c.to_ascii_lowercase())
            .collect();

        if s.is_empty() {
            return true;
        }

        let mut chars = s.chars();

        loop {
            let (Some(l_char), Some(r_char)) = (chars.nth(0), chars.nth_back(0)) else {
                break;
            };

            if l_char != r_char {
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
        let s = "A man, a plan, a canal: Panama".to_owned();

        assert!(Solution::is_palindrome(s));
    }

    #[test]
    fn case_1() {
        let s = "race a car".to_owned();

        assert!(!Solution::is_palindrome(s));
    }

    #[test]
    fn case_2() {
        let s = " ".to_owned();

        assert!(Solution::is_palindrome(s));
    }

    #[test]
    fn case_3() {
        let s = "ab".to_owned();

        assert!(!Solution::is_palindrome(s));
    }
}
