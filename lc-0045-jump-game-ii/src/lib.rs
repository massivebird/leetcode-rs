struct Solution;

#[allow(dead_code, clippy::needless_pass_by_value)]
impl Solution {
    /// My less-than-idiomatic dynamic programming approach.
    ///
    /// Record and optionally overwrite minimum possible jumps to each position.
    pub fn jump(nums: Vec<i32>) -> i32 {
        let mut t = vec![0; nums.len()];

        for (i, jump_capacity) in nums
            .iter()
            .enumerate()
            .map(|(i, j)| (i, usize::try_from(*j).unwrap()))
        {
            for j in ((i + 1)..).take(jump_capacity) {
                if j >= nums.len() {
                    continue;
                }

                let candidate_min = t[i] + 1;
                if j <= i + jump_capacity && t[j] == 0 || candidate_min < t[j] {
                    t[j] = candidate_min;
                }
            }
        }

        t[nums.len() - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_0() {
        assert_eq!(Solution::jump(vec![2, 3, 1, 1, 4]), 2);
    }

    #[test]
    fn case_1() {
        assert_eq!(Solution::jump(vec![2, 3, 0, 1, 4]), 2);
    }
}
