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
        let Some(root) = root else {
            return root;
        };

        match (root.borrow().left.is_some(), root.borrow().right.is_some()) {
            (true, false) => {
                let l = root.borrow_mut().left.take();
                root.borrow_mut().right = l;
            }
            (false, true) => {
                let r = root.borrow_mut().right.take();
                root.borrow_mut().left = r;
            }
            _ => (),
        }

        Self::swap_roots(root.borrow().left.as_ref(), root.borrow().right.as_ref());

        Some(root)
    }

    // TODO:
    // Issue: Asymmetric trees.
    // Maybe a swap(root) controller, which calls either:
    // swap_roots(l, r) or swap_lopsided(root).

    fn swap_roots(l: Option<&Rc<RefCell<TreeNode>>>, r: Option<&Rc<RefCell<TreeNode>>>) {
        match (l, r) {
            (Some(l), Some(r)) => {
                std::mem::swap(&mut l.borrow_mut().val, &mut r.borrow_mut().val);

                Self::swap_roots(l.borrow().left.as_ref(), r.borrow().right.as_ref());
                Self::swap_roots(l.borrow().right.as_ref(), r.borrow().left.as_ref());
            }
            _ => (),
        }
    }
}

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
        ans_root.as_mut().unwrap().borrow_mut().left = ans_child;

        assert_eq!(Solution::invert_tree(root), ans_root);
    }
}
