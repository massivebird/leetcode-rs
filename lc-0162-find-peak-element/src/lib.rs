struct Solution;

#[allow(dead_code)]
impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_0() {
        let nums = vec![1,2,3,1];
        let ans = 2;

        assert_eq!(Solution::find_peak_element(nums), ans);
    }
}
