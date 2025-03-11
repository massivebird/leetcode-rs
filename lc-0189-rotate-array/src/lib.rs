struct Solution;

impl Solution {
    /// Rotates an array `k` times to the right.
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        for _ in 0..k {
            let mut held_val = *nums.iter().last().unwrap();
            for i in 0..nums.len() {
                std::mem::swap(nums.get_mut(i).unwrap(), &mut held_val);
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
        let mut arr = vec![-1, -100, 3, 99];
        Solution::rotate(&mut arr, 2);
        assert_eq!(arr, vec![3, 99, -1, -100]);
    }
}
