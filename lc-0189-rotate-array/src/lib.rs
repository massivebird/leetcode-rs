struct Solution;

impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        for _ in 0..k {
            for i in 0..nums.len() - 1 {
                let this_val = nums[i];
                let next_val = nums[i+1];

                nums[i] = next_val;
                nums[i+1] = this_val;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let mut arr = vec![1, 2, 3, 4, 5, 6, 7];
        Solution::rotate(&mut arr, 3);
        assert_eq!(arr, vec![5, 6, 7, 1, 2, 3, 4]);
    }

    #[test]
    fn case_2() {
        let mut arr = vec![-1,-100,3,99];
        Solution::rotate(&mut arr, 2);
        assert_eq!(arr, vec![3,99,-1,-100]);
    }
}
