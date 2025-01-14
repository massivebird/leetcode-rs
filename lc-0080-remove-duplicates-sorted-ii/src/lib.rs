struct Solution;

impl Solution {
    // MUST perform operations in place without another array
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.len() <= 2 {
            return 2;
        }

        let write_idx = 1;

        for read_idx in 2..nums.len() {
            if nums[read_idx] != nums[read_idx - 2] {
                nums[write_idx] = nums[read_idx];
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
}
