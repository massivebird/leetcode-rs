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

        if root.borrow().left.is_none() && root.borrow().right.is_none() {
            return root.borrow().val;
        }

        let val = root.borrow().val;

        Self::dfs(root.borrow().left.clone(), vec![val])
            + Self::dfs(root.borrow().right.clone(), vec![val])
    }

    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, digits: Vec<i32>) -> i32 {
        let Some(root) = root else {
            return 0;
        };

        let val = root.borrow().val;
        let new_digits = [digits, vec![val]].concat();

        if root.borrow().left.is_none() && root.borrow().right.is_none() {
            return Self::digits_to_val(new_digits);
        }

        Self::dfs(root.borrow().left.clone(), new_digits.clone())
            + Self::dfs(root.borrow().right.clone(), new_digits)
    }

    fn digits_to_val(digits: Vec<i32>) -> i32 {
        let mut sum = 0;

        for (i, val) in digits.iter().rev().enumerate() {
            sum += val * 10_i32.pow(i as u32);
        }

        sum
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
}
