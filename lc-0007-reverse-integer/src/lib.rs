mod tests;

struct Solution;

impl Solution {
    #[allow(dead_code, clippy::missing_const_for_fn)]
    pub fn reverse(x: i32) -> i32 {
        let mut ans: i32 = 0;

        let mut y = x;

        while y.abs() > 0 {
            let ones_digit = y % 10;

            // Before applying arithmetic, check if overflow would occur.
            if ans.checked_mul(10).is_none() {
                return 0;
            }

            ans *= 10;

            if ans.checked_add(ones_digit).is_none() {
                return 0;
            }

            ans += ones_digit;

            y /= 10;
        }

        ans
    }
}
