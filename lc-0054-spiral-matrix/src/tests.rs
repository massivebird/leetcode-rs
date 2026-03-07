#![cfg(test)]

use super::Solution;

#[test]
fn case_0() {
    let matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    let answer = [1, 2, 3, 6, 9, 8, 7, 4, 5];

    assert_eq!(Solution::spiral_order(matrix), answer);
}
