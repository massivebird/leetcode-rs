//! I did it because it was EASY

struct Solution;

impl Solution {
    #[allow(dead_code, clippy::needless_pass_by_value)]
    pub const fn reverse_bits(n: i32) -> i32 {
        n.reverse_bits()
    }
}

#[cfg(test)]
#[allow(clippy::unreadable_literal)]
mod tests {
    use super::*;

    #[test]
    fn case_0() {
        let n = 43261596;
        let ans = 964176192;

        assert_eq!(Solution::reverse_bits(n), ans);
    }
}
