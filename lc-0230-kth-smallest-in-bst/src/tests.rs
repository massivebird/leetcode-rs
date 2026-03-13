#![cfg(test)]

use tree_node::build_tree;

use super::Solution;

#[test]
fn case_0() {
    let root = build_tree(&[Some(3), Some(1), Some(4), None, Some(2)]);
    let k = 1;
    let answer = 1;

    assert_eq!(Solution::kth_smallest(root, k), answer);
}

#[test]
fn case_1() {
    let root = build_tree(&[
        Some(5),
        Some(3),
        Some(6),
        Some(2),
        Some(4),
        None,
        None,
        Some(1),
    ]);
    let k = 3;
    let answer = 3;

    assert_eq!(Solution::kth_smallest(root, k), answer);
}
