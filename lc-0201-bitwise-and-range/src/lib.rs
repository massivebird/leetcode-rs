//! Given two integers `left` and `right` that represent the range `[left, right]`, return the bitwise AND of all numbers in this range, inclusive.
//!
//! MB: Apparently the runtime-optimal solution involves some bitwise dark arts.
//! I'll (totally) do that later.

struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn range_bitwise_and(left: i32, right: i32) -> i32 {
        (left..=right).reduce(|acc, n| acc & n).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_0() {
        let left = 5;
        let right = 7;
        assert_eq!(Solution::range_bitwise_and(left, right), 4);
    }
}
