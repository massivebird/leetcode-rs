#![cfg(test)]
#![allow(clippy::unreadable_literal)]

use super::Solution;

#[test]
fn case_0() {
    let x = -321;
    let expected = -123;

    let result = Solution::reverse(x);

    assert_eq!(result, expected);
}

#[test]
fn case_1() {
    let x = 120;
    let expected = 21;

    let result = Solution::reverse(x);

    assert_eq!(result, expected);
}

#[test]
fn case_2() {
    let x = 1534236469;
    let expected = 0;

    let result = Solution::reverse(x);

    assert_eq!(result, expected);
}
