struct Solution;

#[allow(unused)]
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_0() {
        let x = 121;
        assert!(Solution::is_palindrome(x));
    }

    #[test]
    fn case_1() {
        let x = 124;
        assert!(!Solution::is_palindrome(x));
    }
}
