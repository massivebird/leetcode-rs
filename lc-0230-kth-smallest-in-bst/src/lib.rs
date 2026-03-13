//! Given the root of a binary search tree, and an integer k,
//! return the kth smallest value (1-indexed) of all the values
//! of the nodes in the tree.
//!
//! MB: The most diabolical recursive solution I've ever conceived.
//! Fueled by an air-fried burrito and a banana.
//!
//! I don't collect the tree's values in a vec and then index the vec
//! I SKIPPED that approach because I'm a FREAK OF NATURE I'm a RUNTIME ADDICT
//!
//! Node values are `i32` but are restricted to `0 <= x`,
//! which means the negative space is free to use.

use std::cell::RefCell;
use std::rc::Rc;

// local workspace subcrate
use tree_node::TreeNode;

mod tests;

struct Solution;

impl Solution {
    #[allow(dead_code, clippy::needless_pass_by_value)]
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        Self::rec(root.as_ref(), k)
    }

    /// For some node `n`, returns the following values:
    /// -1: value of `n`'s parent is the solution.
    /// x < 0: solution is `x` nodes away.
    /// x >= 0: the solution is x.
    fn rec(root: Option<&Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        let Some(root) = root.map(|r| r.borrow()) else {
            return -k;
        };

        let mut recv = Self::rec(root.left.as_ref(), k);

        // dbg!((root.val, l));

        // Solution already found in left subtree. Propagate it.
        if !recv.is_negative() {
            return recv;
        } else if recv == -1 {
            return root.val;
        }

        if let Some(r) = root.right.as_ref() {
            let r = Self::rec(Some(r), -recv - 1);
            // dbg!((root.val, r));

            // Solution found in right subtree. Propagate it.
            if r.is_positive() {
                return r;
            }

            recv = (r - 1).max(recv);
        }

        // No solution yet.
        recv + 1
    }
}
