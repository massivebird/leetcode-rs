#![allow(unused)]

struct Solution;

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_0() {
        let nums = vec![1, 2, 3, 1];
        let k = 3;

        assert!(Solution::contains_nearby_duplicate(nums, k));
    }

    #[test]
    fn case_1() {
        let nums = vec![1, 0, 1, 1];
        let k = 1;

        assert!(Solution::contains_nearby_duplicate(nums, k));
    }

    #[test]
    fn case_2() {
        let nums = vec![1, 2, 3, 1, 2, 3];
        let k = 2;

        assert!(!Solution::contains_nearby_duplicate(nums, k));
    }
}
