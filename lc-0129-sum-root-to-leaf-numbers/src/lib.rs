use array_to_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

struct Solution;

#[allow(dead_code, clippy::needless_pass_by_value)]
impl Solution {
    // Recursively determines if there exists a path from the tree's root to
    // some leaf node such that the path's sum equals `target_sum`.
    //
    // Node values _and_ `target_sum` are signed values!
    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let Some(root) = root else {
            return 0;
        };

        let val = root.borrow().val;

        // Short-circuit if the tree has only one node.
        // I couldn't figure out how to bake this into the dfs!
        if Self::leaf(&root) {
            return val;
        }

        Self::dfs(root.borrow().left.clone(), vec![val])
            + Self::dfs(root.borrow().right.clone(), vec![val])
    }

    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, digits: Vec<i32>) -> i32 {
        let Some(root) = root else {
            // Should only occur when searching a non-leaf's non-existent child.
            return 0;
        };

        let val = root.borrow().val;
        let new_digits = [digits, vec![val]].concat();

        // If this is a leaf node, then compile and return the value.
        if Self::leaf(&root) {
            return Self::digits_to_val(new_digits);
        }

        Self::dfs(root.borrow().left.clone(), new_digits.clone())
            + Self::dfs(root.borrow().right.clone(), new_digits)
    }

    fn digits_to_val(digits: Vec<i32>) -> i32 {
        let mut sum = 0;

        for (i, val) in digits.iter().rev().enumerate() {
            sum += val * 10_i32.pow(u32::try_from(i).unwrap());
        }

        sum
    }

    fn leaf(node: &Rc<RefCell<TreeNode>>) -> bool {
        node.borrow().left.is_none() && node.borrow().right.is_none()
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused)]
    use super::Solution;
    use array_to_tree::build_tree;

    #[test]
    fn case_0() {
        let arr = vec![Some(1), Some(2), Some(3)];
        let ans = 25;

        let tree = build_tree(&arr);
        assert_eq!(Solution::sum_numbers(tree), ans);
    }

    #[test]
    fn case_1() {
        let arr = vec![Some(4), Some(9), Some(0), Some(5), Some(1)];
        let ans = 1026;

        let tree = build_tree(&arr);
        assert_eq!(Solution::sum_numbers(tree), ans);
    }

    #[test]
    fn case_2() {
        let arr = vec![Some(9)];
        let ans = 9;

        let tree = build_tree(&arr);
        assert_eq!(Solution::sum_numbers(tree), ans);
    }
}
