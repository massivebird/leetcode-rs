struct Solution;

#[allow(unused, clippy::needless_pass_by_value)]
impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_0() {
        let digits = vec![1, 2, 3];
        let ans = vec![1, 2, 4];

        assert_eq!(Solution::plus_one(digits), ans);
    }

    #[test]
    fn case_1() {
        let digits = vec![9];
        let ans = vec![1, 0];

        assert_eq!(Solution::plus_one(digits), ans);
    }

    #[test]
    fn case_2() {
        let digits = vec![1, 9];
        let ans = vec![2, 0];

        assert_eq!(Solution::plus_one(digits), ans);
    }
}
