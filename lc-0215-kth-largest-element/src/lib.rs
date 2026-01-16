struct Solution;

#[allow(dead_code)]
impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_0() {
        let nums = vec![3, 2, 1, 5, 6, 4];
        let k = 2;
        let ans = 5;

        assert_eq!(Solution::find_kth_largest(nums, k), ans);
    }
}
