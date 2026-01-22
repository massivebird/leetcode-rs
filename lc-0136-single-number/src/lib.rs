struct Solution;

#[allow(dead_code, clippy::needless_pass_by_value)]
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        // Big idea:
        // (1) Each possible value has a unique bit pattern.
        // (2) A XOR A cancels itself out and equals zero.
        // (3) A XOR B XOR A equals B.

        let mut screen: i32 = 0;

        for n in nums {
            screen ^= n;
        }

        screen
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_0() {
        assert_eq!(Solution::single_number(vec![2, 2, 1]), 1);
    }

    #[test]
    fn case_1() {
        assert_eq!(Solution::single_number(vec![4, 1, 2, 1, 2]), 4);
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::single_number(vec![1]), 1);
    }
}
