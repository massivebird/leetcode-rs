struct Solution {}

#[allow(dead_code, clippy::needless_pass_by_value)]
impl Solution {
    // Start at `nums[0]`. The value at `nums[x]` represents the maximum distance
    // you can jump forwards from position `x`.
    //
    // Is it possible to jump to the final element in `nums`?
    pub fn can_jump(nums: Vec<i32>) -> bool {
        // `navigable[x] = 1` means that index `x` is navigable from some
        // other index.
        let mut navigable = vec![0; nums.len()];
        // Start at the first index.
        navigable[0] = 1;

        for (idx, max_jump) in nums.into_iter().enumerate() {
            for jdx in (idx + 1)..=(idx + max_jump as usize) {
                if let Some(v) = navigable.get_mut(jdx) {
                    *v = 1;
                }
            }
        }

        navigable.iter().all(|v| v == &1)
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
}
