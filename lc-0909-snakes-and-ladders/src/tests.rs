#![cfg(test)]

use super::Solution;

#[test]
fn case_0() {
    let board = vec![
        vec![-1, -1, -1, -1, -1, -1],
        vec![-1, -1, -1, -1, -1, -1],
        vec![-1, -1, -1, -1, -1, -1],
        vec![-1, 35, -1, -1, 13, -1],
        vec![-1, -1, -1, -1, -1, -1],
        vec![-1, 15, -1, -1, -1, -1],
    ];
    let answer = 4;

    assert_eq!(Solution::snakes_and_ladders(board), answer);
}
