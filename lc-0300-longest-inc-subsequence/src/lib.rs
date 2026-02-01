//! Given an integer array nums, return the length of the longest strictly increasing subsequence.
//! The subsequence does not have to be contiguous.
//!
//! MB: My dynamic programming approach!
//!
//! The time complexity is slower than O(n).

mod tests;

struct Solution;

impl Solution {
    #[allow(dead_code, clippy::needless_pass_by_value)]
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        // Value at index `i` represents the length of the longest increasing
        // subsequence starting at index `i`.
        let mut lengths: Vec<usize> = vec![1; nums.len()];

        let mut ans = 1;

        for (i, this) in nums.iter().enumerate().rev() {
            for (j, other) in nums.iter().enumerate().skip(i + 1) {
                if other <= this {
                    continue;
                }

                lengths[i] = lengths[i].max(1 + lengths[j]);
            }

            ans = ans.max(lengths[i]);
        }

        ans.try_into().unwrap()
    }
}
