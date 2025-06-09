// --- DO NOT EDIT

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
    #[allow(clippy::missing_const_for_fn, clippy::use_self)]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

// --- OK NOW YOU CAN EDIT

use std::cell::RefCell;
use std::rc::Rc;

struct Solution;

#[allow(unused, clippy::needless_pass_by_value)]
impl Solution {
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        if p.is_none() && q.is_none() {
            return true;
        } else if matches!((p.clone(), q.clone()), (Some(_), None) | (None, Some(_))) {
            return false;
        }

        let p_head = p.unwrap();
        let q_head = q.unwrap();

        if p_head.borrow().val != q_head.borrow().val {
            return false;
        }

        Self::is_same_tree(p_head.borrow().left.clone(), q_head.borrow().left.clone()) &&
        Self::is_same_tree(p_head.borrow().right.clone(), q_head.borrow().right.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_0() {
        let p = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: None,
            }))),
        })));

        let q = p.clone();

        assert!(Solution::is_same_tree(p, q));
    }
}
