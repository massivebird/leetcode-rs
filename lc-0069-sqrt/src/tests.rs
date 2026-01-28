#![cfg(test)]

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

#[test]
fn case_2() {
    let x = 0;
    let ans = 0;

    assert_eq!(Solution::my_sqrt(x), ans);
}

#[test]
fn case_3() {
    let x = 2;
    let ans = 1;

    assert_eq!(Solution::my_sqrt(x), ans);
}

#[test]
fn case_4() {
    let x = 3;
    let ans = 1;

    assert_eq!(Solution::my_sqrt(x), ans);
}

#[test]
fn case_5() {
    let x = 2_147_395_599;
    let ans = 46339;

    assert_eq!(Solution::my_sqrt(x), ans);
}
