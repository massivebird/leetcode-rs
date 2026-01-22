struct Solution;

// 0198: House Robber
//
// There is a sequence of houses. You can't rob adjacent houses. Find the
// maximum amount of cash you can rob from these fools.
//
// The value at `nums[i]` represents the amount of cash in the i-th house.

impl Solution {
    #[allow(dead_code, clippy::needless_pass_by_value)]
    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return nums[0];
        }

        let mut arr = vec![0; nums.len()];

        arr[0] = nums[0];
        arr[1] = i32::max(nums[0], nums[1]);

        for i in 2..nums.len() {
            arr[i] = i32::max(arr[i - 2] + nums[i], arr[i - 1]);
        }

        arr[nums.len() - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_0() {
        let nums = vec![1, 2, 3, 1];
        let ans = 4;

        assert_eq!(Solution::rob(nums), ans);
    }

    #[test]
    fn case_1() {
        let nums = vec![2, 7, 9, 3, 1];
        let ans = 12;

        assert_eq!(Solution::rob(nums), ans);
    }

    #[test]
    fn case_2() {
        let nums = vec![0];
        let ans = 0;

        assert_eq!(Solution::rob(nums), ans);
    }
}
