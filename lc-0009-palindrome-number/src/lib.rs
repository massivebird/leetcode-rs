struct Solution;

#[allow(unused, clippy::missing_const_for_fn)]
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x.is_negative() {
            return false;
        } else if x < 10 {
            return true;
        }

        let mut x = x;

        let highest_ten_power = (x as f32).log10() as i32;

        let mut digits: Vec<i32> = Vec::new();

        for i in (0..=highest_ten_power).rev() {
            let get_digit = |pos: i32| {
                if pos == 0 {
                    x - (x / 10) * 10
                } else {
                    x / 10i32.pow(pos as u32)
                }
            };

            let digit = get_digit(i);

            if digit > 0 {
                x -= 10i32.pow(i as u32) * digit;

                digits.push(digit);
            } else {
                digits.push(0);
            }
        }

        for l in 0..digits.len() {
            let r = digits.len() - 1 - l;

            if digits[l] != digits[r] {
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
