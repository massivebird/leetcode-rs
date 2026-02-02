use list_node::ListNode;

mod tests;

struct Solution;

impl Solution {
    #[allow(dead_code, clippy::needless_pass_by_value)]
    pub fn rotate_right(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        head.as_ref()?; // Short-circuit if head is None

        // Convert linked list to an array because I'm not sauced the hell up
        let mut v: Vec<i32> = {
            let mut buf = Vec::new();

            let mut next = head.as_ref();

            while let Some(node) = next {
                buf.push(node.val);
                next = node.next.as_ref();
            }

            buf
        };

        // Simplify large `k` values.
        let k = usize::try_from(k).unwrap() % v.len();

        // Rotate array `k` times to the right.
        for _ in 0..k {
            let mut held_val = v[v.len() - 1];
            for i in 0..v.len() {
                std::mem::swap(v.get_mut(i).unwrap(), &mut held_val);
            }
        }

        // Convert the rotated array back to a linked list.
        Self::build_rec(&v, 0)
    }

    fn build_rec(nums: &[i32], i: usize) -> Option<Box<ListNode>> {
        let mut node = ListNode::new(*nums.get(i)?);

        node.next = Self::build_rec(nums, i + 1);

        Some(Box::new(node))
    }
}
