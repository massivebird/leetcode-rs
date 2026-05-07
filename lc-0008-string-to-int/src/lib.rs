//! [link](https://leetcode.com/problems/string-to-integer-atoi/description/)
//!
//! Inspired by C's `atoi`, I think.
//!
//! Algorithm details:
//!
//! 1. Whitespace: Ignore any leading whitespace (" ").
//! 2. Signedness: Determine the sign by checking if the next character is '-' or '+', assuming positivity if neither present.
//! 3. Conversion: Skip leading zeros until a non-digit character is encountered.
//!     + If no digits were read, then the result is 0.
//! 4. Rounding: Saturate within the i32 range [-2^{31}, 2^{31} - 1].
//!
//! MB: dead simple iterator work. Finally got to do some `char as u8` arithmetic!!

mod tests;

struct Solution;

impl Solution {
    #[allow(dead_code, clippy::needless_pass_by_value)]
    pub fn my_atoi(s: String) -> i32 {
        let mut chars = s.trim().chars().peekable();

        let is_negative = chars.peek().is_some_and(|c| c == &'-');

        // If there's a leading sign character '+' or '-', eat it.
        if is_negative || chars.peek().is_some_and(|c| c == &'+') {
            chars.next();
        }

        let mut ans: i32 = 0;

        for c in chars {
            if !c.is_numeric() {
                break;
            }

            ans = ans.saturating_mul(10);

            if is_negative {
                ans = ans.saturating_sub(i32::from(c as u8 - b'0'));
            } else {
                ans = ans.saturating_add(i32::from(c as u8 - b'0'));
            }
        }

        ans
    }
}
