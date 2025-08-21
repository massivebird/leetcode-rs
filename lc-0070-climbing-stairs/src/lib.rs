struct Solution;

#[allow(unused)]
impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_0() {
        let n = 2;
        let ans = 2;

        assert_eq!(Solution::climb_stairs(n), ans);
    }

    #[test]
    fn case_1() {
        let n = 3;
        let ans = 3;

        assert_eq!(Solution::climb_stairs(n), ans);
    }

    #[test]
    fn case_2() {
        let n = 4;
        let ans = 5;

        assert_eq!(Solution::climb_stairs(n), ans);
    }

    #[test]
    fn case_3() {
        let n = 5;
        let ans = 8;

        assert_eq!(Solution::climb_stairs(n), ans);
    }
}
