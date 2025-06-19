struct Solution;

#[allow(unused, clippy::missing_const_for_fn)]
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        // Some short circuit solutions
        if x.is_negative() {
            return false;
        } else if x < 10 {
            return true;
        }

        let mut x = x;

        // Equal to `number of digits - 1`.
        #[allow(clippy::cast_possible_truncation, clippy::cast_precision_loss)]
        let highest_ten_power = (x as f32).log10() as i32;

        // Will contain the input's individual digits.
        let mut digits: Vec<i32> = Vec::new();

        for i in (0..=highest_ten_power)
            .rev()
            .map(|v| u32::try_from(v).unwrap())
        {
            let digit = x / 10i32.pow(i);

            if digit > 0 {
                digits.push(digit);
                // Prepare `x` for the next digit retrieval.
                x -= 10i32.pow(i) * digit;
            } else {
                digits.push(0);
            }
        }

        // Verify that digits vector is symmetric.
        for l_idx in 0..(digits.len() / 2) {
            let r_idx = digits.len() - 1 - l_idx;

            if digits[l_idx] != digits[r_idx] {
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
        let x = 121;
        assert!(Solution::is_palindrome(x));
    }

    #[test]
    fn case_1() {
        let x = 10;
        assert!(!Solution::is_palindrome(x));
    }

    #[test]
    fn case_2() {
        let x = -121;
        assert!(!Solution::is_palindrome(x));
    }

    #[test]
    fn case_3() {
        let x = 5;
        assert!(Solution::is_palindrome(x));
    }

    #[test]
    fn case_4() {
        let x = 0;
        assert!(Solution::is_palindrome(x));
    }

    #[test]
    fn case_5() {
        let x = 313;
        assert!(Solution::is_palindrome(x));
    }

    #[test]
    fn case_6() {
        let x = 1122;
        assert!(!Solution::is_palindrome(x));
    }

    #[test]
    fn case_7() {
        let x = 1001;
        assert!(Solution::is_palindrome(x));
    }

    #[test]
    fn case_8() {
        let x = 9999;
        assert!(Solution::is_palindrome(x));
    }
}
