// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    #[must_use]
    pub const fn new(val: i32) -> Self {
        Self {
            val,
            left: None,
            right: None,
        }
    }
}

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;

#[allow(dead_code)]
impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let Some(root) = root else { return true };

        Self::eval_subtrees(root.borrow().left.as_ref(), root.borrow().right.as_ref())
    }

    fn eval_subtrees(l: Option<&Rc<RefCell<TreeNode>>>, r: Option<&Rc<RefCell<TreeNode>>>) -> bool {
        match (l, r) {
            (Some(l), Some(r)) => {
                if l.borrow().val != r.borrow().val {
                    return false;
                }

                // "Symmetric" means we compare the outer nodes with each other, and
                // mutatis mutandis the inner nodes.
                // i.e. Compare the left subtree's left child with the
                // right subtree's right child.
                Self::eval_subtrees(l.borrow().left.as_ref(), r.borrow().right.as_ref())
                    && Self::eval_subtrees(l.borrow().right.as_ref(), r.borrow().left.as_ref())
            }
            // Mutual nonexistence satisfies symmetry.
            (None, None) => true,
            // Missing a corresponding subtree -> not symmetric!
            _ => false,
        }
    }
}
