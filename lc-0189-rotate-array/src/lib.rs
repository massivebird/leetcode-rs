struct Solution;

impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let mut hand_og_idx: usize = 0;
        let mut hand: i32 = nums[0];

        // Each value displaces some other value.
        // Let's move the first one, pick up whatever it
        // displaces, then move that one.
        // BUUUUUTTTTT even displacement displaces only a SUBSET

        for _ in 0..nums.len() {
            let new_idx = (hand_og_idx + k as usize) % nums.len();

            // Swap displaced value with hand
            std::mem::swap(&mut nums[new_idx], &mut hand);
            hand_og_idx = new_idx;
            println!("{nums:?}")
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
