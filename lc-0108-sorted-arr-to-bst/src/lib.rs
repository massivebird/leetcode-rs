use std::cell::RefCell;
use std::rc::Rc;
use tree_node::TreeNode;

struct Solution;

impl Solution {
    #[allow(dead_code, clippy::needless_pass_by_value, clippy::unnecessary_wraps)]
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let root_idx = nums.len() / 2;

        let mut root = TreeNode::new(nums[root_idx]);

        // Left subtree
        for (i, n) in nums.iter().copied().take(root_idx).enumerate() {
            let mut node = TreeNode::new(n);

            let old_left = root.left.take();

            // Time to rotate!
            if old_left.is_some() && i % 2 == 0 {
                old_left.clone().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(node)));
                root.left = old_left;
            } else {
                node.left = old_left;
                root.left = Some(Rc::new(RefCell::new(node)));
            }
        }

        // Right subtree
        for (i, n) in nums.iter().copied().skip(root_idx + 1).enumerate() {
            let mut node = TreeNode::new(n);

            let old_right = root.right.take();

            // Time to rotate!
            if old_right.is_some() && i % 2 == 0 {
                old_right.clone().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(node)));
                root.right = old_right;
            } else {
                node.left = old_right;
                root.right = Some(Rc::new(RefCell::new(node)));
            }
        }

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
        let expected = build_tree(&[Some(3), Some(1)]);

        assert_eq!(actual, expected);
    }

    #[test]
    fn case_1() {
        let nums = vec![-10, -3, 0, 5, 9];
        let actual = Solution::sorted_array_to_bst(nums);
        let expected = build_tree(&[Some(0), Some(-3), Some(9), Some(-10), None, Some(5)]);

        assert_eq!(actual, expected);
    }

    #[test]
    fn case_2() {
        let nums = vec![0, 1, 2, 3, 4, 5];
        let actual = Solution::sorted_array_to_bst(nums);
        let expected = build_tree(&[Some(3), Some(1), Some(5), Some(0), Some(2), Some(4)]);

        assert_eq!(actual, expected);
    }

    #[test]
    fn case_3() {
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

    #[test]
    fn case_4() {
        let nums = vec![0, 1, 2, 3, 4, 5, 6, 7];
        let actual = Solution::sorted_array_to_bst(nums);
        let expected = build_tree(&[
            Some(4),
            Some(2),
            Some(6),
            Some(1),
            Some(3),
            Some(5),
            Some(7),
            Some(0),
        ]);

        assert_eq!(actual, expected);
    }
}
