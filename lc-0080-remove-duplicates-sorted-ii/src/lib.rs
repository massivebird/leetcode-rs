struct Solution;

impl Solution {
    // MUST perform operations in place without another array
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.len() <= 2 {
            return 2;
        }

        let mut write_idx = 2;

        for read_idx in 3..nums.len() {
            let this = nums[read_idx];
            // If this is equal to previous two, read next.
            if this == nums[read_idx - 1] && this == nums[read_idx - 2] {
                write_idx += 1;
                continue;
            }

            // If this is equal to one of previous two, inc write idx.
            if this == nums[read_idx - 1] || this == nums[read_idx - 2] {
                write_idx += 1;
                continue;
            }

            nums[write_idx] = this;
            write_idx += 1;
        }

        dbg!(nums);
        write_idx as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let mut nums = vec![1, 1, 1, 2, 2, 3];
        assert_eq!(Solution::remove_duplicates(&mut nums), 5);
        assert!(nums.starts_with(&[1, 1, 2, 2, 3]));
    }

    #[test]
    fn case_2() {
        let mut nums = vec![0, 0, 1, 1, 1, 1, 2, 3, 3];
        assert_eq!(Solution::remove_duplicates(&mut nums), 7);
        assert!(nums.starts_with(&[0, 0, 1, 1, 2, 3, 3]));
    }
}
