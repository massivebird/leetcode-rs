// The input vector contains the digits of a number.
// Return the equivalent vector of the input value plus one.

struct Solution;

#[allow(unused, clippy::needless_pass_by_value)]
impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut digits: Vec<i32> = digits.into_iter().rev().collect();

        let mut idx = 0;

        let mut carry = false;

        loop {
            let digit = digits.get_mut(idx);

            match digit {
                Some(&mut 9) => {
                    *digit.unwrap() = 0;
                    carry = true;
                }
                Some(x) if carry => {
                    *x += 1;
                    break;
                }
                Some(x) => {
                    *x += 1;
                    break;
                }
                None if carry => {
                    digits = [digits, vec![1]].concat();
                    break;
                }
                _ => break,
            }

            idx += 1;
        }

        digits.into_iter().rev().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_0() {
        let digits = vec![1, 2, 3];
        let ans = vec![1, 2, 4];

        assert_eq!(Solution::plus_one(digits), ans);
    }

    #[test]
    fn case_1() {
        let digits = vec![9];
        let ans = vec![1, 0];

        assert_eq!(Solution::plus_one(digits), ans);
    }

    #[test]
    fn case_2() {
        let digits = vec![1, 9];
        let ans = vec![2, 0];

        assert_eq!(Solution::plus_one(digits), ans);
    }
}
