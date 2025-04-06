struct Solution {}

#[allow(dead_code, clippy::needless_pass_by_value)]
impl Solution {
    // Start at `nums[0]`. The value at `nums[x]` represents the maximum distance
    // you can jump forwards from position `x`.
    //
    // Is it possible to jump to the final element in `nums`?
    pub fn can_jump(nums: Vec<i32>) -> bool {
        if nums.len() <= 1 {
            return true;
        }

        // Invariant: ALL indices must be navigable in order to reach the end.
        //
        // If there is some unreachable index anywhere, then we cannot jump
        // to it or any other index thereafter, including the end.
        //
        // The contrapositive of the above is if we can reach the end, then
        // all indices are reachable.

        let mut farthest_navigable_idx = 0;

        for (i, dist) in nums.iter().enumerate().take(nums.len() - 1) {
            // Check if this index is unreachable.
            if farthest_navigable_idx < i {
                return false;
            }

            farthest_navigable_idx = usize::max(farthest_navigable_idx, i + *dist as usize);

            // Check if final index (or beyond) is reachable.
            if farthest_navigable_idx >= nums.len() - 1 {
                return true;
            }
        }

        // All positions have been exhausted, but the final index is still
        // unreachable.
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_0() {
        assert!(Solution::can_jump(vec![2, 3, 1, 1, 4]));
    }

    #[test]
    fn case_1() {
        assert!(!Solution::can_jump(vec![3, 2, 1, 0, 4]));
    }

    #[test]
    fn case_2() {
        assert!(Solution::can_jump(vec![0]));
    }

    #[test]
    fn case_3() {
        assert!(Solution::can_jump(vec![2, 0]));
    }

    #[test]
    fn case_4() {
        assert!(Solution::can_jump(vec![2, 0, 0]));
    }

    #[test]
    fn case_5() {
        assert!(!Solution::can_jump(vec![0, 2, 3]));
    }
}
