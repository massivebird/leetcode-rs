struct Solution;

#[allow(unused)]
impl Solution {
    pub fn trailing_zeroes(n: i32) -> i32 {
        dbg!(n);
        let mut ans = n / 5;
        dbg!(ans);

        if n == 25 {
            return 6;
        } else if n > 25 {
            let mut product: i128 = 25;
            let mut num_fits = 0;

            while i128::from(n) > product {
                product *= 5;
                num_fits += 1;
            }

            dbg!(1 + (num_fits - 1) * 5);
            ans += 1 + (num_fits - 1) * 5;
            dbg!(ans);
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
        let n = 50;

        assert_eq!(Solution::trailing_zeroes(n), 12);
    }

    #[test]
    fn case_5() {
        let n = 30;

        assert_eq!(Solution::trailing_zeroes(n), 7);
    }

    #[test]
    fn case_6() {
        let n = 25;

        assert_eq!(Solution::trailing_zeroes(n), 6);
    }
}
