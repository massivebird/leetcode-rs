struct Solution;

#[allow(unused)]
impl Solution {
    pub fn trailing_zeroes(n: i32) -> i32 {
        let fac: i128 = (1i128..=i128::from(n)).product();

        if fac % 10 != 0 {
            return 0;
        }

        let mut n = 1;

        let mut ans: i32 = 0;

        while fac % 10i128.pow(n) == 0 {
            ans = n as i32;
            n += 1;
        }

        ans
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

    #[test]
    fn case_2() {
        let n = 7; // -> 5040

        assert_eq!(Solution::trailing_zeroes(n), 1);
    }

    #[test]
    fn case_3() {
        let n = 13;

        assert_eq!(Solution::trailing_zeroes(n), 2);
    }

    #[test]
    fn case_4() {
        let n = 40;

        assert_eq!(Solution::trailing_zeroes(n), 9);
    }
}
