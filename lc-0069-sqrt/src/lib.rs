//! Given a non-negative integer x, return the square root of x rounded down to
//! the nearest integer. The returned integer should be non-negative as well.
//!
//! MB: TLDR this challenge kinda sucks
//!
//! I built this binary search solution but ran into integer overflow issues.
//! So I tried pivoting to Newton's method, but it didn't work with large
//! numbers, even with infinite iterations.
//!
//! Turns out my initial approach _was_ right, I just had to cast the
//! intermediate test values to a wider data type. This is a language-
//! dependent issue. psh. Whatever

mod tests;

struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn my_sqrt(x: i32) -> i32 {
        let x: i64 = x.into();
        if x < 2 {
            return i32::try_from(x).unwrap();
        }

        let mut l: i64 = 0;
        let mut r: i64 = x / 2;

        // Binary search!
        loop {
            let mid = l.midpoint(r);

            let test = (mid + 1) * (mid + 1);

            if (mid + 1).pow(2) > x && mid.pow(2) < x {
                return i32::try_from(mid).unwrap();
            }

            if test == x {
                return i32::try_from(mid).unwrap();
            } else if test > x {
                r = mid;
            } else if l == mid {
                l = r;
            } else {
                l = mid;
            }
        }
    }
}
