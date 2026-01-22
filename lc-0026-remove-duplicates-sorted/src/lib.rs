struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        let mut new_nums: Vec<i32> = vec![nums[0]];

        let mut prev_uniq: i32 = nums[0];

        for e in nums.iter().skip(1) {
            if *e != prev_uniq {
                prev_uniq = *e;
                new_nums.push(*e);
            }
        }

        let num_uniques = new_nums.len();

        *nums = new_nums;

        i32::try_from(num_uniques).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let mut nums = vec![1, 1, 2];
        assert_eq!(Solution::remove_duplicates(&mut nums), 2);
        assert_eq!(nums, vec![1, 2]);
    }

    #[test]
    fn case_2() {
        let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        assert_eq!(Solution::remove_duplicates(&mut nums), 5);
        assert_eq!(nums, vec![0, 1, 2, 3, 4]);
    }
}
