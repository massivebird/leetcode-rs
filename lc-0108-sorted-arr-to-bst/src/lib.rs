use std::cell::RefCell;
use std::rc::Rc;
use tree_node::TreeNode;

struct Solution;

impl Solution {
    #[allow(dead_code, clippy::needless_pass_by_value)]
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use tree_node::build_tree;

    use super::*;

    #[test]
    fn case_0() {
        let nums = vec![1, 3];
        let actual = Solution::sorted_array_to_bst(nums);
        let expected = build_tree(&[Some(1), None, Some(3)]);

        assert_eq!(actual, expected);
    }
}
