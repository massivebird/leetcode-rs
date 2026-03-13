#![cfg(test)]

use tree_node::build_tree;

use super::Solution;

#[test]
fn case_0() {
    let root = build_tree(&[Some(1), Some(2), Some(3), None, Some(5), None, Some(4)]);
    let ans = [1, 3, 4];

    assert_eq!(Solution::right_side_view(root), ans);
}
