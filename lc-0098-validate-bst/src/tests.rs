#![cfg(test)]

use tree_node::build_tree;

use super::Solution;

#[test]
fn case_0() {
    let root = build_tree(&[Some(2), Some(1), Some(3)]);
    assert!(Solution::is_valid_bst(root));
}
