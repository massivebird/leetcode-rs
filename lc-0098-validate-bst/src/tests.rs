#![cfg(test)]

use tree_node::build_tree;

use super::Solution;

#[test]
fn case_0() {
    let root = build_tree(&[Some(2), Some(1), Some(3)]);
    assert!(Solution::is_valid_bst(root));
}

#[test]
fn case_1() {
    let root = build_tree(&[Some(5), Some(1), Some(4), None, None, Some(3), Some(6)]);
    assert!(!Solution::is_valid_bst(root));
}

#[test]
fn case_2() {
    let root = build_tree(&[
        Some(32),
        Some(26),
        Some(47),
        Some(19),
        None,
        None,
        Some(56),
        None,
        Some(27),
    ]);
    assert!(!Solution::is_valid_bst(root));
}
