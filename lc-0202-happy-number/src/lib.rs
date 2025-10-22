#[allow(unused)]
struct Solution;

impl Solution {
    #[allow(unused)]
    pub fn is_happy(n: i32) -> bool {
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_0() {
        assert!(Solution::is_happy(19));
    }

    #[test]
    fn case_1() {
        assert!(!Solution::is_happy(2));
    }
}
