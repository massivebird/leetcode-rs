struct Solution;

#[allow(unused)]
impl Solution {
    // One possible conceptualization: you are counting the leaf nodes of the
    // tree(s) representing all possible navigations through the stairs.
    pub fn climb_stairs(n: i32) -> i32 {
        if n <= 3 {
            return n;
        }

        let n: usize = usize::try_from(n).unwrap();

        // The value at the nth step corresponds to the number of ways
        // that step can be reached.
        //
        // Base cases:
        // - One way to reach the first step (don't think about it).
        // - Two ways to reach the second step, either:
        //   - Stepped distance of 1 on first step, or
        //   - Stepped distance of 2 on first step.
        let mut stairs = vec![0; n];
        stairs[0] = 1;
        stairs[1] = 2;

        for i in 2..n {
            stairs[i] = stairs[i - 1] + stairs[i - 2];
        }

        stairs[n - 1]
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
