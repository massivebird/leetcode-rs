// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  #[must_use]
  pub const fn new(val: i32) -> Self {
    Self {
      val,
      left: None,
      right: None
    }
  }
}

use std::rc::Rc;
use std::cell::RefCell;

struct Solution;

#[allow(dead_code, clippy::needless_pass_by_value)]
impl Solution {
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused)]
    use super::Solution;

    #[test]
    fn case_0() {
        todo!()
    }
}
