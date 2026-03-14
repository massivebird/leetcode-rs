//! Given the root of a binary tree, determine if it is a valid BST.
//!
//! MB: inefficient but functional solution! For every node, I evaluate
//! the min and max values in its subtrees.
//!
//! TODO: add min and max args to rec fn and propagate constraints
//! _downwards_ into subtrees.

use std::cell::RefCell;
use std::rc::Rc;

use tree_node::TreeNode;

mod tests;

struct Solution;

impl Solution {
    #[allow(dead_code, clippy::needless_pass_by_value)]
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let Some(root) = root.as_ref() else {
            return false;
        };

        Self::rec(root)
    }

    fn rec(root: &Rc<RefCell<TreeNode>>) -> bool {
        let val = root.borrow().val;

        let mut valid = true;

        if let Some(l) = root.borrow().left.as_ref() {
            // dbg!((val, Self::eval(l, true)));
            if Self::eval(l, true) >= val || Self::eval(l, false) >= val {
                return false;
            }

            valid &= Self::rec(l);
        }

        if let Some(r) = root.borrow().right.as_ref() {
            // dbg!((val, Self::eval(r, false)));
            if Self::eval(r, false) <= val || Self::eval(r, true) <= val {
                return false;
            }

            valid &= Self::rec(r);
        }

        valid
    }

    fn eval(root: &Rc<RefCell<TreeNode>>, min: bool) -> i32 {
        let mut p = root.borrow().val;

        let f = if min { i32::min } else { i32::max };

        if let Some(l) = root.borrow().left.as_ref() {
            p = f(p, Self::eval(l, min));
        }

        if let Some(r) = root.borrow().right.as_ref() {
            p = f(p, Self::eval(r, min));
        }

        p
    }
}
