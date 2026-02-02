#![allow(dead_code)]

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    #[must_use]
    pub const fn new(val: i32) -> Self {
        Self { next: None, val }
    }
}

#[must_use]
pub fn build(nums: &[i32]) -> Option<Box<ListNode>> {
    build_rec(nums, 0)
}

fn build_rec(nums: &[i32], i: usize) -> Option<Box<ListNode>> {
    let mut node = ListNode::new(*nums.get(i)?);

    node.next = build_rec(nums, i + 1);

    Some(Box::new(node))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_0() {
        let a = build(&[1, 2, 3]);
        let ans = ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode { val: 3, next: None })),
            })),
        };

        assert_eq!(a, Some(Box::new(ans)));
    }

    #[test]
    fn case_1() {
        assert_eq!(build(&[]), None);
    }
}
