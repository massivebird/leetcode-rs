use std::cell::RefCell;
use std::rc::Rc;
use tree_node::TreeNode;

struct Solution;

impl Solution {
    #[allow(dead_code, clippy::needless_pass_by_value)]
    pub fn get_minimum_difference(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_0() {
        let root =
            tree_node::build_tree(&[Some(1), Some(0), Some(48), None, None, Some(12), Some(49)]);

        assert_eq!(Solution::get_minimum_difference(root), 1);
    }
}
