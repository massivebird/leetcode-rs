use std::cell::RefCell;
use std::rc::Rc;
use tree_node::TreeNode;

mod tests;

struct Solution;

impl Solution {
    #[allow(dead_code, clippy::needless_pass_by_value)]
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        use std::collections::VecDeque;

        if root.is_none() {
            return Vec::new();
        }

        // BFS queue. Item=(Node, 0-based level)
        let mut queue: VecDeque<(Rc<RefCell<TreeNode>>, usize)> = vec![(root.unwrap(), 0)].into();

        let mut output: Vec<Vec<i32>> = Vec::new();

        while let Some((node, lvl)) = queue.pop_front() {
            let val = node.borrow().val;

            if let Some(lvl_items) = output.get_mut(lvl) {
                lvl_items.push(val);
            } else {
                output.push(vec![val]);
            }

            if let Some(left) = &node.borrow().left {
                queue.push_back((left.clone(), lvl + 1));
            }

            if let Some(right) = &node.borrow().right {
                queue.push_back((right.clone(), lvl + 1));
            }
        }

        output
    }
}
