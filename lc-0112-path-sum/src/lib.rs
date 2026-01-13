use tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

struct Solution;

#[allow(dead_code, clippy::needless_pass_by_value)]
impl Solution {
    // Recursively determines if there exists a path from the tree's root to
    // some leaf node such that the path's sum equals `target_sum`.
    //
    // Node values _and_ `target_sum` are signed values!
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        let Some(root) = root else {
            return false;
        };

        let val = root.borrow().val;
        let new = target_sum - val;

        if new == 0 && root.borrow().left.is_none() && root.borrow().right.is_none() {
            return true;
        }

        Self::has_path_sum(root.borrow().left.clone(), new)
            || Self::has_path_sum(root.borrow().right.clone(), new)
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused)]
    use super::Solution;
    use tree_node::build_tree;

    #[test]
    fn case_0() {
        let arr = vec![Some(-2), None, Some(-3)];
        let target_sum = -5;

        let tree = build_tree(&arr);
        assert!(Solution::has_path_sum(tree, target_sum));
    }

    #[test]
    fn case_1() {
        let arr = vec![Some(1), Some(-2), Some(3)];
        let target_sum = 3;

        let tree = build_tree(&arr);
        assert!(!Solution::has_path_sum(tree, target_sum));
    }
}
