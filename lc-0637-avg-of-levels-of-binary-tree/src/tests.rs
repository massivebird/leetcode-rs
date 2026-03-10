#![cfg(test)]

use super::Solution;

#[test]
fn case_0() {
    let root =
    tree_node::build_tree(&[Some(3), Some(9), Some(20), None, None, Some(15), Some(7)]);

    assert_eq!(
        Solution::average_of_levels(root),
        [3.00000, 14.50000, 11.00000]
    );
}
