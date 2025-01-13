struct Solution;

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let mut a = vec![3, 2, 2, 3];
        assert_eq!(Solution::remove_element(&mut a, 3), 2);
    }
}
