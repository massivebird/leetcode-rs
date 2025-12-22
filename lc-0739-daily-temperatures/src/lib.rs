struct Solution;

#[allow(dead_code)]
impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_0() {
        let temps = vec![73, 74, 75, 71, 69, 72, 76, 73];
        let answer = vec![1, 1, 4, 2, 1, 1, 0, 0];
        assert_eq!(Solution::daily_temperatures(temps), answer);
    }
}
