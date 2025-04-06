struct Solution {}

#[allow(dead_code, clippy::needless_pass_by_value)]
impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        if nums.len() <= 1 {
            return true;
        }

        for d in 1..=*nums.first().unwrap() {
            let dest = d as usize;

            if Self::jump(&nums, dest) {
                return true;
            }
        }

        false
    }

    fn jump(nums: &Vec<i32>, start: usize) -> bool {
        // If we can max jump OOB, then we can make it to the end.
        let Some(&max_jump) = nums.get(start) else {
            return true;
        };

        if start as i32 + max_jump >= nums.len() as i32 - 1 {
            return true;
        }

        for d in 1..=max_jump {
            if Self::jump(nums, start + d as usize) {
                return true;
            }
        }

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
}
