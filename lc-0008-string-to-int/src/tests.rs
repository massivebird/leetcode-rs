#![cfg(test)]

use super::Solution;

#[test]
fn case_0() {
    let s = "42".to_string();
    let expected = 42;

    let result = Solution::my_atoi(s);

    assert_eq!(result, expected);
}

#[test]
fn case_1() {
    let s = "-0042".to_string();
    let expected = -42;

    let result = Solution::my_atoi(s);

    assert_eq!(result, expected);
}

#[test]
fn case_2() {
    let s = "+1".to_string();
    let expected = 1;

    let result = Solution::my_atoi(s);

    assert_eq!(result, expected);
}
