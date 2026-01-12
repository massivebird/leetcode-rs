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
use std::collections::VecDeque;
use std::rc::Rc;

#[must_use]
/// Converts an array of nodes into a tree.
///
/// # Examples
///
/// ```ignore
/// \[1, 2, 3]
///
///   1
///  / \
/// 2   3
/// ```
///
/// ```ignore
/// [4, 5, null, 6, 7]
///
///     4
///    /
///   5 
///  / \
/// 6   7
/// ```
///
/// # Panics
///
/// Could panic I guess. LMAO I don't think it can?? But for the sake of
/// "safety" and "completeness": I warned you
pub fn build_tree(vec: &[Option<i32>]) -> Option<Rc<RefCell<TreeNode>>> {
    let mut stack: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();

    let mut vec_iter = vec.iter();

    let Some(Some(root_val)) = vec_iter.next() else {
        return None;
    };

    let root_node = Rc::new(RefCell::new(TreeNode::new(*root_val)));
    stack.push_back(root_node.clone());
    let tree = Some(root_node);

    let mut left = true;

    for node in vec_iter {
        let node = node
            .as_ref()
            .map(|val| Rc::new(RefCell::new(TreeNode::new(*val))));

        if let Some(node) = node.clone() {
            stack.push_back(node);
        }

        if left {
            stack.front().unwrap().borrow_mut().left = node;
        } else {
            stack.front().unwrap().borrow_mut().right = node;
            stack.pop_front();
        }
        left ^= true; // Flip bool
    }

    tree
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_0() {
        dbg!(build_tree(&[Some(1), Some(2), None, None, Some(3)]));
    }
}
