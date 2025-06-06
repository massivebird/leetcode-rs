struct Solution;

#[allow(clippy::needless_pass_by_value, unused)]
impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_0() {
        let numbers = vec![2, 7, 11, 15];
        let target = 9;

        assert_eq!(Solution::two_sum(numbers, target), vec![1, 2]);
    }
}
