//! Given the root of a binary tree, return the average value of the nodes on each level in the form of an array. Answers within 10^{-5} of the actual answer will be accepted.
//!
//! MB: yuh huh

use std::cell::RefCell;
use std::rc::Rc;
use tree_node::TreeNode;

mod tests;

struct Solution;

#[allow(dead_code)]
impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn average_of_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<f64> {
        // Indexed by 0-based level: (level sum, # nodes in level)
        let mut sum_len: Vec<(i64, u32)> = Vec::new();

        Self::traverse(root.as_ref(), 0, &mut sum_len);

        let mut ans: Vec<f64> = Vec::new();

        for (sum, len) in sum_len {
            #[allow(clippy::cast_precision_loss)]
            ans.push(sum as f64 / f64::from(len));
        }

        ans
    }

    fn traverse(root: Option<&Rc<RefCell<TreeNode>>>, level: usize, sum_len: &mut Vec<(i64, u32)>) {
        let Some(root) = root else {
            return;
        };

        if let Some((sum, num)) = sum_len.get_mut(level) {
            *sum += i64::from(root.borrow().val);
            *num += 1;
        } else {
            sum_len.push((i64::from(root.borrow().val), 1));
        }

        Self::traverse(root.borrow().left.as_ref(), level + 1, sum_len);
        Self::traverse(root.borrow().right.as_ref(), level + 1, sum_len);
    }
}
