struct Solution;

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut a = vec![1, 2, 3, 0, 0, 0];
        let mut b = vec![2, 5, 6];

        Solution::merge(&mut a, 6, &mut b, 3);

        assert_eq!(a, vec![1, 2, 2, 3, 5, 6]);
    }
}
