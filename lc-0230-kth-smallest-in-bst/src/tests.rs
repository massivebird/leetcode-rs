#![cfg(test)]

use tree_node::build_tree;

use super::Solution;

#[test]
fn case_0() {
    let root = build_tree(&[Some(4), Some(2), Some(5), None, Some(3)]);
    let k = 1;
    let answer = 2;

    assert_eq!(Solution::kth_smallest(root, k), answer);
}

#[test]
fn case_1() {
    let root = build_tree(&[
        Some(6),
        Some(4),
        Some(7),
        Some(3),
        Some(5),
        None,
        None,
        Some(2),
    ]);
    let k = 3;
    let answer = 4;

    assert_eq!(Solution::kth_smallest(root, k), answer);
}

#[test]
fn case_2() {
    let root = build_tree(&[Some(5), None, Some(6)]);
    let k = 2;
    let answer = 6;

    assert_eq!(Solution::kth_smallest(root, k), answer);
}

#[test]
fn case_3() {
    let root = build_tree(&[Some(4), Some(2), Some(5), None, Some(3)]);
    let k = 3;
    let answer = 4;

    assert_eq!(Solution::kth_smallest(root, k), answer);
}
