//! Given the root of a binary tree, imagine yourself standing on the
//! right side of it, return the values of the nodes you can see
//! ordered from top to bottom.
//!
//! MB: very very simple in-order bfs approach.
//!
//! There is a vector v, where `v[i]` holds a value from some tree node at
//! level `i`.
//!
//! If each node at level `i` writes its value to `v[i]`, then `v[i]` will
//! eventually contain the value of the level's rightmost node. This is
//! thanks to the *in-order* bfs traversal.

use std::cell::RefCell;
use std::rc::Rc;
use tree_node::TreeNode;

mod tests;

struct Solution;

impl Solution {
    #[allow(dead_code, clippy::needless_pass_by_value)]
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        if root.is_none() {
            return Vec::new();
        }

        let mut answer: Vec<i32> = Vec::new();

        Self::bfs(root.as_ref().unwrap(), 0, &mut answer);

        answer
    }

    fn bfs(root: &Rc<RefCell<TreeNode>>, level: usize, answer: &mut Vec<i32>) {
        if let Some(cell) = answer.get_mut(level) {
            *cell = root.borrow().val;
        } else {
            answer.push(root.borrow().val);
        }

        if let Some(left) = root.borrow().left.as_ref() {
            Self::bfs(left, level + 1, answer);
        }

        if let Some(right) = root.borrow().right.as_ref() {
            Self::bfs(right, level + 1, answer);
        }
    }
}
