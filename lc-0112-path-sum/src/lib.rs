use array_to_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

struct Solution;

#[allow(dead_code, clippy::needless_pass_by_value)]
impl Solution {
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        Self::src_rec(root.as_ref(), target_sum)
    }

    fn src_rec(root: Option<&Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        let Some(root) = root else {
            return false;
        };

        let val = root.borrow().val;
        let new = match (target_sum.signum(), val.signum()) {
            (_, 0) => target_sum,
            // Travel away from zero with opposite signs
            (1, -1) => target_sum - val,
            (-1, 1) => target_sum - val,

            // Travel towards zero with same signs
            (-1, -1) => target_sum - val,
            (0, 1) => target_sum - val,
            (1, 1) => target_sum - val,
            (0, -1) => target_sum - val,

            (_, _) => unreachable!(),
        };

        if new == 0 && root.borrow().left.is_none() && root.borrow().right.is_none() {
            return true;
        }

        Self::src_rec(root.borrow().left.as_ref(), new)
            || Self::src_rec(root.borrow().right.as_ref(), new)
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused)]
    use super::Solution;
    use array_to_tree::build_tree;

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
