struct Solution;

#[allow(unused)]
impl Solution {
    pub fn trailing_zeroes(n: i32) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_0() {
        let n = 3; // -> 6

        assert_eq!(Solution::trailing_zeroes(n), 0);
    }

    #[test]
    fn case_1() {
        let n = 5; // -> 120

        assert_eq!(Solution::trailing_zeroes(n), 1);
    }
}
