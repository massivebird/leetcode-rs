struct Solution;

impl Solution {
    #[allow(dead_code, clippy::needless_pass_by_value)]
    pub fn candy(ratings: Vec<i32>) -> i32 {
        use std::cmp::Ordering;

        let mut ans: i32 = 1;

        let mut prev_trend = Ordering::Equal;
        let mut streak: i32 = 0;

        // These two are used when evaluating a streak of `Less` immediately after
        // a `Greater` in order to keep that `Greater` a local maximum.
        let mut latest_greater_val: i32 = 0;
        let mut less_streak_after_greater = true;

        for (i, rating) in ratings.iter().enumerate().skip(1) {
            ans += 1;

            let this_trend = rating.cmp(&ratings[i - 1]);

            match (prev_trend, this_trend) {
                (Ordering::Less, Ordering::Less) => {
                    streak += 1;
                    ans += streak;

                    if less_streak_after_greater
                        && streak >= latest_greater_val - 1
                        && latest_greater_val > 1
                    {
                        // dbg!("additional");
                        ans += 1;
                    }
                }
                (Ordering::Less | Ordering::Equal, Ordering::Greater) => {
                    streak = 1;
                    ans += 1;
                }
                (Ordering::Equal, Ordering::Less) => {
                    streak = 1;
                    less_streak_after_greater = false;
                    ans += streak;
                }
                (Ordering::Greater, Ordering::Less) => {
                    latest_greater_val = streak + 1;
                    less_streak_after_greater = true;
                    streak = 0;
                }
                (Ordering::Greater, Ordering::Greater) => {
                    streak += 1;
                    ans += streak;
                }
                _ => (),
            }

            // dbg!(this_trend);
            // dbg!(ans);

            prev_trend = this_trend;
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_0() {
        assert_eq!(Solution::candy(vec![1, 0, 2]), 5);
    }

    #[test]
    fn case_1() {
        assert_eq!(Solution::candy(vec![1, 2, 2]), 4);
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::candy(vec![1, 2, 3, 4]), 10);
    }

    #[test]
    fn case_3() {
        assert_eq!(Solution::candy(vec![1, 3, 2, 2, 1]), 7);
    }

    #[test]
    fn case_4() {
        assert_eq!(Solution::candy(vec![1, 2, 87, 87, 87, 2, 1]), 13);
    }

    #[test]
    fn case_5() {
        assert_eq!(Solution::candy(vec![1, 2, 3, 1, 0]), 9);
    }

    #[test]
    fn case_6() {
        assert_eq!(Solution::candy(vec![4, 3, 2, 1]), 10);
    }

    #[test]
    fn case_7() {
        assert_eq!(Solution::candy(vec![1, 6, 10, 8, 7, 3, 2]), 18);
    }

    #[test]
    fn case_8() {
        assert_eq!(Solution::candy(vec![6, 10, 8, 7, 3, 2]), 16);
    }

    #[test]
    fn case_9() {
        assert_eq!(
            Solution::candy(vec![1, 2, 3, 5, 4, 3, 2, 1, 4, 3, 2, 1]),
            31
        );
    }

    #[test]
    fn case_10() {
        assert_eq!(Solution::candy(vec![1, 4, 3, 2, 1]), 11);
    }

    #[test]
    fn case_11() {
        assert_eq!(
            Solution::candy(vec![
                1, 2, 3, 5, 4, 3, 2, 1, 4, 3, 2, 1, 3, 2, 1, 1, 2, 3, 4, 4, 3, 2, 1
            ]),
            57
        );
    }
}
