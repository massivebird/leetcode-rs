struct Solution;

impl Solution {
    #[allow(unused, clippy::needless_pass_by_value)]
    pub fn is_palindrome(s: String) -> bool {
        let mut l: usize = 0;
        let mut r: usize = s.len() - 1;

        while l <= r {

        }

        todo!()
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
}
