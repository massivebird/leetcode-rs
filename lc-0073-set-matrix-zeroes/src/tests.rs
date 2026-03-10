#![cfg(test)]

use super::Solution;

#[test]
fn case_0() {
    let mut matrix = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];
    let answer = [[1, 0, 1], [0, 0, 0], [1, 0, 1]];

    Solution::set_zeroes(&mut matrix);

    assert_eq!(matrix, answer);
}

#[test]
fn case_1() {
    let mut matrix = vec![vec![0, 1, 2, 0], vec![3, 4, 5, 2], vec![1, 3, 1, 5]];
    let answer = [[0, 0, 0, 0], [0, 4, 5, 0], [0, 3, 1, 0]];

    Solution::set_zeroes(&mut matrix);

    assert_eq!(matrix, answer);
}

