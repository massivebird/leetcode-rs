struct Solution;

impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let mut hand_og_idx: usize = 0;
        let mut hand: i32 = nums[0];

        // Each value displaces some other value.
        // Let's move the first one, pick up whatever it
        // displaces, then move that one.

        for _ in 0..nums.len() {

        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let mut arr = vec![1,2,3,4,5,6,7];
        Solution::rotate(&mut arr, 3);
        assert_eq!(arr, vec![5,6,7,1,2,3,4]);
    }
}
