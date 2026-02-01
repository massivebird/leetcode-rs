#![cfg(test)]

use super::*;

#[test]
fn case_0() {
    let nums = vec![10, 9, 2, 5, 3, 7, 101, 18];
    let ans = 4;

    assert_eq!(Solution::length_of_lis(nums), ans);
}

#[test]
fn case_1() {
    let nums = vec![7, 7, 7, 7, 7, 7, 7];
    let ans = 1;

    assert_eq!(Solution::length_of_lis(nums), ans);
}

#[test]
fn case_2() {
    let nums = vec![0, 1, 0, 3, 2, 3];
    let ans = 4;

    assert_eq!(Solution::length_of_lis(nums), ans);
}
