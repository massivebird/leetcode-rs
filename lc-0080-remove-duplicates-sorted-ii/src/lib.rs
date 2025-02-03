struct Solution;

impl Solution {
    // MUST perform operations in place without another array
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.len() <= 2 {
            return nums.len() as i32;
        }

        let mut write_idx = 2;
        let mut latest_uniq: i32 = nums[1];
        let mut latest_uniq_capped: bool = nums[0] == nums[1];

        for read_idx in 2..nums.len() {
            if write_idx >= nums.len() {
                break;
            }

            let this = nums[read_idx];

            // We have enough of this number.
            if this == latest_uniq && latest_uniq_capped {
                continue;
            }

            // Second of the latest number. No more after this!
            if this == latest_uniq && !latest_uniq_capped {
                latest_uniq_capped = true;
                nums[write_idx] = this;
                write_idx += 1;
                continue;
            }

            // New number!
            if this != latest_uniq {
                latest_uniq = this;
                latest_uniq_capped = false;
                nums[write_idx] = this;
                write_idx += 1;
                continue;
            }
        }

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
