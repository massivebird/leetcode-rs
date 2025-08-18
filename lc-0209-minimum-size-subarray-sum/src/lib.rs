#[allow(dead_code)]
struct Solution;

impl Solution {
    #[allow(clippy::needless_pass_by_value, dead_code)]
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            if nums[0] >= target {
                return 1;
            }

            return 0;
        }

        if nums[0] >= target {
            return 1;
        }

        let mut active_sum = nums[0] + nums[1];

        let mut best_len = 0;

        let mut ldx = 0;
        let mut window_len = 2;

        let try_shift_ldx = |ldx: usize| {
            if ldx == nums.len() - 1 {
                return Err(());
            }

            Ok(ldx + 1)
        };

        let try_shift_window = |ldx: usize, window_len: usize| {
            if ldx + window_len == nums.len() {
                return Err(());
            }

            Ok(window_len + 1)
        };

        loop {
            // println!("{ldx} {window_len} {active_sum}");
            match active_sum.cmp(&target) {
                std::cmp::Ordering::Less => {
                    match try_shift_window(ldx, window_len) {
                        Ok(v) => window_len = v,
                        Err(()) => return best_len,
                    }

                    active_sum += nums[ldx + window_len - 1];
                }
                std::cmp::Ordering::Equal => {
                    if best_len == 0 {
                        best_len = window_len as i32;
                    } else {
                        best_len = i32::min(best_len, i32::try_from(window_len).unwrap());
                    }

                    // Shift entire window right by one.

                    // Subtract leftmost value.
                    active_sum -= nums[ldx];

                    match try_shift_ldx(ldx) {
                        Ok(v) => ldx = v,
                        Err(()) => return best_len,
                    }

                    // Exit if window is OOB.
                    if ldx + window_len > nums.len() {
                        return best_len;
                    }

                    // Add new rightmost value.
                    active_sum += nums[ldx + window_len - 1];
                }
                std::cmp::Ordering::Greater => {
                    if best_len == 0 {
                        best_len = window_len as i32;
                    } else {
                        best_len = i32::min(best_len, i32::try_from(window_len).unwrap());
                    }

                    active_sum -= nums[ldx];

                    match try_shift_ldx(ldx) {
                        Ok(v) => ldx = v,
                        Err(()) => return best_len,
                    }

                    window_len -= 1;
                }
            }
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
