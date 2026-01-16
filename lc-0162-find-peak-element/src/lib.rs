struct Solution;

#[allow(dead_code)]
impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return 0;
        }

        // Short-circuit if the edges are valid peaks.
        if nums.len() >= 2 && (nums[0] > nums[1] || nums[nums.len() - 1] > nums[nums.len() - 2]) {
            if nums[0] > nums[1] {
                return 0;
            } else if nums[nums.len() - 1] > nums[nums.len() - 2] {
                return i32::try_from(nums.len() - 1).unwrap();
            }
        }

        // Since we just checked if nums[0] was a peak, we know it's less than
        // or equal to nums[1].
        // So if we start by analyzing nums[1], we only have to check its
        // right side.
        // If we're clever (and we are), we can continue this trend
        // for the entire input vector.

        let mut i = 1;

        while let Some(this) = nums.get(i) {
            if nums.get(i + 1).is_some_and(|right| right < this) {
                // Both sides are lesser than current value.
                // Found a peak!
                return i32::try_from(i).unwrap();
            } else if nums.get(i + 1).is_some_and(|right| right > this) {
                // Only right side is greater.
                // Let's check it next!
                i += 1;
                continue;
            }

            // Both sides are lesser or equal.
            // Right side definitely won't be a peak.
            // We can skip it.
            i += 2;
        }

        unreachable!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_0() {
        let nums = vec![1, 2, 3, 1];
        let ans = 2;

        assert_eq!(Solution::find_peak_element(nums), ans);
    }

    #[test]
    fn case_1() {
        let nums = vec![1];
        let ans = 0;

        assert_eq!(Solution::find_peak_element(nums), ans);
    }

    #[test]
    fn case_2() {
        let nums = vec![2, 1];
        let ans = 0;

        assert_eq!(Solution::find_peak_element(nums), ans);
    }
}
