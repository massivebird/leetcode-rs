struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn hamming_weight(n: i32) -> i32 {
        i32::try_from(format!("{n:b}").chars().filter(|c| *c == '1').count()).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_0() {
        assert_eq!(Solution::hamming_weight(11), 3);
    }

    #[test]
    fn case_1() {
        assert_eq!(Solution::hamming_weight(128), 1);
    }
}
