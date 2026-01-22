struct Solution;

impl Solution {
    // MUST perform operations in place without another array
    #[allow(dead_code, clippy::needless_pass_by_value, clippy::ptr_arg)]
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.len() <= 2 {
            return i32::try_from(nums.len()).unwrap();
        }

        let mut write_idx = 2;

        for read_idx in 2..nums.len() {
            if nums[read_idx] != nums[write_idx - 2] {
                nums[write_idx] = nums[read_idx];
                write_idx += 1;
            }
        }

        // dbg!(&nums);
        i32::try_from(write_idx).unwrap()
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
