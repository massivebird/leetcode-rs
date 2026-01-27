//! Given the root of a Binary Search Tree (BST), return the minimum absolute
//! difference between the values of any two different nodes in the tree.

use std::cell::RefCell;
use std::rc::Rc;
use tree_node::TreeNode;

struct Solution;

impl Solution {
    /// Operates under the premise that in-order traversal of a BST returns
    /// its values in increasing order.
    #[allow(dead_code, clippy::needless_pass_by_value)]
    pub fn get_minimum_difference(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        // `with_capacity` is the new `new`
        let mut in_order: Vec<i32> = Vec::with_capacity(root.iter().len());

        Self::eval_in_order(root.as_ref().unwrap(), &mut in_order);

        let mut ans = i32::MAX;

        for (i, val) in in_order.iter().enumerate() {
            let Some(next) = in_order.get(i + 1) else {
                break;
            };

            ans = ans.min(next - val);
        }

        ans
    }

    /// Evaluates nodes in the following order:
    /// (1) Left child
    /// (2) Self
    /// (3) Right child
    fn eval_in_order(node: &Rc<RefCell<TreeNode>>, in_order: &mut Vec<i32>) {
        if let Some(left) = &node.borrow().left {
            Self::eval_in_order(left, in_order);
        }

        // Push this node's value unless it's a duplicate.
        let val = node.borrow().val;
        if in_order.last().is_none_or(|v| *v != val) {
            in_order.push(val);
        }

        if let Some(right) = &node.borrow().right {
            Self::eval_in_order(right, in_order);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_0() {
        let root =
            tree_node::build_tree(&[Some(1), Some(0), Some(48), None, None, Some(12), Some(49)]);

        assert_eq!(Solution::get_minimum_difference(root), 1);
    }

    #[test]
    fn case_1() {
        let root = tree_node::build_tree(&[Some(1), None, Some(5)]);

        assert_eq!(Solution::get_minimum_difference(root), 4);
    }
}
