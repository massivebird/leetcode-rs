struct Solution;

#[allow(unused, clippy::missing_const_for_fn)]
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x.is_negative() {
            return false;
        } else if x < 10 {
            return true;
        }

        let highest_ten_power = (x as f32).log10() as i32;

        for l in 0..=highest_ten_power {
            let r = highest_ten_power - l;

            let get_digit = |pos: i32| {
                if pos == 0 {
                    x - (x / 10) * 10
                } else {
                    x / 10i32.pow(pos as u32)
                }
            };

            let l_digit = get_digit(l);
            let r_digit = get_digit(r);

            dbg!((l, r));
            dbg!((l_digit, r_digit));

            if l_digit != r_digit {
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
        let x = 124;
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
}
