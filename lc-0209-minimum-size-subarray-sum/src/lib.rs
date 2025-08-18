#[allow(dead_code)]
struct Solution;

impl Solution {
    #[allow(clippy::needless_pass_by_value, dead_code)]
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        // Handle short-circuit opportunities and single-element input arrays.
        if nums[0] >= target {
            return 1;
        } else if nums.len() == 1 {
            return 0;
        }

        // Sliding window properties.
        let mut ldx: usize = 0;
        let mut window_len: usize = 2;

        // Tracking sum of the values within the sliding window.
        let mut window_sum: i32 = nums[0] + nums[1];

        // The shortest subarray length recorded so far.
        // `0` indicates no valid subarray.
        let mut best_len: i32 = 0;

        loop {
            if window_sum < target {
                // Window sum needs to increase.
                //
                // Extend window rightwards to eat up more values.

                // Bounds check: is the window already on the edge?
                if ldx + window_len == nums.len() {
                    return best_len;
                }

                window_len += 1;

                window_sum += nums[ldx + window_len - 1];

                continue;
            }

            // Window sum is sufficiently large.

            if best_len == 0 {
                best_len = i32::try_from(window_len).unwrap();
            } else {
                best_len = i32::min(best_len, i32::try_from(window_len).unwrap());
            }

            // Since the window sum is sufficiently large, try reducing it.
            //
            // Shrink the sliding window by shifting its left bound forward.

            window_sum -= nums[ldx];

            // Bounds check: is the left idx already on the rightmost element?
            if ldx == nums.len() - 1 {
                return best_len;
            }

            ldx += 1;
            window_len -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_0() {
        let target = 7;
        let nums = vec![2, 3, 1, 2, 4, 3];
        let ans = 2;

        assert_eq!(Solution::min_sub_array_len(target, nums), ans);
    }

    #[test]
    fn case_1() {
        let target = 4;
        let nums = vec![1, 4, 4];
        let ans = 1;

        assert_eq!(Solution::min_sub_array_len(target, nums), ans);
    }

    #[test]
    fn case_2() {
        let target = 11;
        let nums = vec![1, 2, 3, 4, 5];
        let ans = 3;

        assert_eq!(Solution::min_sub_array_len(target, nums), ans);
    }

    #[test]
    fn case_3() {
        let target = 7;
        let nums = vec![5];
        let ans = 0;

        assert_eq!(Solution::min_sub_array_len(target, nums), ans);
    }

    #[test]
    fn case_4() {
        let target = 5;
        let nums = vec![7];
        let ans = 1;

        assert_eq!(Solution::min_sub_array_len(target, nums), ans);
    }
}
