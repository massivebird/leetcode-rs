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
impl Solution {
    #[allow(clippy::needless_pass_by_value, dead_code)]
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        todo!();
    }
}
