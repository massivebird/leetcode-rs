#![cfg(test)]

use tree_node::build_tree;

use super::Solution;

#[test]
fn case_0() {
    let root = build_tree(&[Some(3), Some(9), Some(20), None, None, Some(15), Some(7)]);
    let ans = vec![vec![3], vec![9, 20], vec![15, 7]];

    assert_eq!(Solution::level_order(root), ans);
}
