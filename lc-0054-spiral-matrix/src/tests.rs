#![cfg(test)]

use super::Solution;

#[test]
fn case_0() {
    let matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    let answer = [1, 2, 3, 6, 9, 8, 7, 4, 5];

    assert_eq!(Solution::spiral_order(matrix), answer);
}

#[test]
fn case_1() {
    let matrix = vec![vec![1]];
    let answer = [1];

    assert_eq!(Solution::spiral_order(matrix), answer);
}

#[test]
fn case_2() {
    let matrix = vec![];
    let answer = [];

    assert_eq!(Solution::spiral_order(matrix), answer);
}

#[test]
fn case_3() {
    let matrix = vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![9, 10, 11, 12]];
    let answer = [1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7];

    assert_eq!(Solution::spiral_order(matrix), answer);
}
