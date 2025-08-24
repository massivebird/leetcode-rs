struct Solution;

#[allow(unused, clippy::needless_pass_by_value)]
impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_0() {
        let height = vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];
        let ans = 6;

        assert_eq!(Solution::trap(height), ans);
    }

    #[test]
    fn case_1() {
        let height = vec![4, 2, 0, 3, 2, 5];
        let ans = 9;

        assert_eq!(Solution::trap(height), ans);
    }
}
