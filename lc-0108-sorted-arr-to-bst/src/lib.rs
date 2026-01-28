//! Given an integer array nums where the elements are sorted in ascending
//! order, convert it to a height-balanced binary search tree.
//!
//! MB: My solution operates on the premise that the tree is rooted at the
//! array's middle element. This premise holds as you recursively split
//! the array in half to form the root's left and right subtrees.

use std::cell::RefCell;
use std::rc::Rc;
use tree_node::TreeNode;

struct Solution;

impl Solution {
    #[allow(dead_code, clippy::needless_pass_by_value, clippy::unnecessary_wraps)]
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::subtree(0, nums.len() - 1, &nums)
    }

    /// Recursively build the BST.
    /// `l` and `r` are the inclusive index bounds of the subtree.
    fn subtree(l: usize, r: usize, nums: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        // Return early if all elements have been exhausted.
        if l > r {
            return None;
        }

        let m = l.midpoint(r);

        let mut root = TreeNode::new(nums[m]);

        // Check if there are only one or two elements left to capture.
        // These edge cases cause problems if uncaught.
        if l == r {
            return Some(Rc::new(RefCell::new(root)));
        } else if l == r - 1 {
            root.right = Some(Rc::new(RefCell::new(TreeNode::new(nums[r]))));
            return Some(Rc::new(RefCell::new(root)));
        }

        root.left = Self::subtree(l, m.saturating_sub(1), nums);
        root.right = Self::subtree(m + 1, r, nums);

        Some(Rc::new(RefCell::new(root)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tree_node::build_tree;

    // BTW! There can be multiple acceptable answers, but I'll only write one
    // expected answer per case.

    #[test]
    fn case_0() {
        let nums = vec![1, 3];
        let actual = Solution::sorted_array_to_bst(nums);
        let expected = build_tree(&[Some(1), None, Some(3)]);

        assert_eq!(actual, expected);
    }

    #[test]
    fn case_1() {
        let nums = vec![0, 1, 2, 3, 4, 5, 6];
        let actual = Solution::sorted_array_to_bst(nums);
        let expected = build_tree(&[
            Some(3),
            Some(1),
            Some(5),
            Some(0),
            Some(2),
            Some(4),
            Some(6),
        ]);

        assert_eq!(actual, expected);
    }
}
