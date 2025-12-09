// Given definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

// Given definition for a binary tree node.
impl TreeNode {
    #[inline]
    #[must_use]
    #[allow(clippy::use_self, clippy::missing_const_for_fn)]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;

#[allow(clippy::needless_pass_by_value, dead_code)]
impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::eval_subtree(root.as_ref())
    }

    fn eval_subtree(root: Option<&Rc<RefCell<TreeNode>>>) -> i32 {
        let Some(root) = root else {
            return 0;
        };

        let left_depth = Self::eval_subtree(root.borrow().left.as_ref());
        let right_depth = Self::eval_subtree(root.borrow().right.as_ref());

        // I wrote this solution on accident.
        // The more conventional solution increment the depth within each function call.
        // This method cascades the depth upward! Or whatever you want to call it.
        1 + left_depth.max(right_depth)
    }
}
