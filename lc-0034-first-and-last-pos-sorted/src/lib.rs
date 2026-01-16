struct Solution;

#[allow(dead_code)]
impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        use std::cmp::Ordering;

        if nums.is_empty() {
            return vec![-1, -1];
        }

        let mut l: usize = 0;
        let mut r: usize = nums.len() - 1;

        // Short-circuit if this element does not fit within the bounds
        // of this array.
        if target > nums[r] || target < nums[l] {
            return vec![-1, -1];
        }

        let mut i;

        loop {
            i = l.midpoint(r);
            // dbg!((l, r, i));

            let test = nums[i];

            match test.cmp(&target) {
                Ordering::Equal => {
                    return Self::target_bounds(&nums, i);
                }

                // Search has completed without finding the target.
                Ordering::Less if l == i => {
                    // Weird edge case:
                    // SOMETIMES the value to the right will be the target.
                    if nums.get(i + 1).is_some_and(|v| *v == target) {
                        return Self::target_bounds(&nums, i + 1);
                    }

                    // But not all the time.
                    return vec![-1, -1];
                }
                Ordering::Greater if r == i => break,

                // More searching required!
                Ordering::Less => l = i,
                Ordering::Greater => r = i,
            }
        }

        vec![-1, -1]
    }

    /// Returns the 0-based index bounds containing the target value.
    fn target_bounds(nums: &[i32], i: usize) -> Vec<i32> {
        let target = nums[i];

        let mut ans: Vec<i32> = vec![i32::try_from(i).unwrap(), i32::try_from(i).unwrap()];

        // Keep checking leftwards until a non-target.
        for offset in 1.. {
            let Some(l) = i.checked_sub(offset) else {
                break;
            };

            if nums.get(l).is_some_and(|v| *v == target) {
                ans[0] = i32::try_from(l).unwrap();
            } else {
                break;
            }
        }

        // Keep checking rightwards until a non-target.
        for offset in 1.. {
            let Some(r) = i.checked_add(offset) else {
                break;
            };

            if nums.get(r).is_some_and(|v| *v == target) {
                ans[1] = i32::try_from(r).unwrap();
            } else {
                break;
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_0() {
        let nums = vec![5, 7, 7, 8, 8, 10];
        let target = 8;
        let ans = [3, 4];

        assert_eq!(Solution::search_range(nums, target), ans);
    }

    #[test]
    fn case_1() {
        let nums = vec![];
        let target = 0;
        let ans = [-1, -1];

        assert_eq!(Solution::search_range(nums, target), ans);
    }

    #[test]
    fn case_2() {
        let nums = vec![1, 4];
        let target = 4;
        let ans = [1, 1];

        assert_eq!(Solution::search_range(nums, target), ans);
    }

    #[test]
    fn case_3() {
        let nums = vec![5, 7, 7, 8, 8, 10];
        let target = 6;
        let ans = [-1, -1];

        assert_eq!(Solution::search_range(nums, target), ans);
    }
}
