struct Solution;

impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let mut arr = vec![1,2,3,4,5,6,7];
        Solution::rotate(&mut arr, 3);
        assert_eq!(arr, vec![5,6,7,1,2,3,4]);
    }
}
