#![cfg(test)]

use super::Solution;

#[test]
fn it_works() {
    let mut matrix = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];
    let answer = [[1, 0, 1], [0, 0, 0], [1, 0, 1]];

    Solution::set_zeroes(&mut matrix);

    assert_eq!(matrix, answer);
}
