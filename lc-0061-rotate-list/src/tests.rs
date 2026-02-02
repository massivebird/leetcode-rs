#![cfg(test)]

use super::*;

#[test]
fn case_0() {
    let head = list_node::build(&[1, 2, 3, 4, 5]);
    let k = 2;
    let ans = list_node::build(&[4, 5, 1, 2, 3]);

    assert_eq!(Solution::rotate_right(head, k), ans);
}

#[test]
fn case_1() {
    assert_eq!(Solution::rotate_right(None, 1), None);
}
