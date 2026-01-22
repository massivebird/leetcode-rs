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
use std::cell::RefCell;
use std::rc::Rc;

struct Solution;

#[allow(clippy::needless_pass_by_value, dead_code)]
impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::swap_rec(root.as_ref());

        root
    }

    fn swap_rec(root: Option<&Rc<RefCell<TreeNode>>>) {
        let Some(root) = root else {
            return;
        };

        // Swap left and right subtrees, whether `Some(_)` or `None`.
        let l = root.borrow_mut().left.take();
        let r = std::mem::replace(&mut root.borrow_mut().right, l);
        root.borrow_mut().left = r;

        // Repeat this process for both subtrees.
        Self::swap_rec(root.borrow().left.as_ref());
        Self::swap_rec(root.borrow().right.as_ref());
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused)]
    use super::{Rc, RefCell, Solution, TreeNode};

    #[test]
    fn case_0() {
        let mut root = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        let child = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        root.as_mut().unwrap().borrow_mut().left = child;

        let mut ans_root = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        let ans_child = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        ans_root.as_mut().unwrap().borrow_mut().right = ans_child;

        assert_eq!(Solution::invert_tree(root), ans_root);
    }
}
