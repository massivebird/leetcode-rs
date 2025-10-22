#[allow(unused)]
struct Solution;

impl Solution {
    #[allow(unused)]
    pub fn is_happy(mut n: i32) -> bool {
        let mut known_values = std::collections::HashSet::new();

        loop {
            let mut sum = 0;

            n = loop {
                // Get rightmost digit.
                let digit = n % 10;

                sum += digit * digit;

                // Right shift `n` by one (1) base 10 digit.
                // e.g. 241 -> 24
                n /= 10;

                // Check if all digits have been exhausted.
                if n <= 0 {
                    break sum;
                }
            };

            if n == 1 {
                return true;
            }

            if !known_values.insert(n) {
                break false;
            }
        }
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

    #[test]
    fn case_2() {
        assert!(Solution::is_happy(7));
    }
}
