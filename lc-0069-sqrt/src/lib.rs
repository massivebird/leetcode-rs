struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn my_sqrt(x: i32) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_0() {
        let x = 4;
        let ans = 2;

        assert_eq!(Solution::my_sqrt(x), ans);
    }

    #[test]
    fn case_1() {
        let x = 8;
        let ans = 2;

        assert_eq!(Solution::my_sqrt(x), ans);
    }
}
