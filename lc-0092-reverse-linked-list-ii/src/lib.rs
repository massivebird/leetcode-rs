// --- DO NOT EDIT

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    #[allow(unused, clippy::use_self, clippy::missing_const_for_fn)]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

// --- OK NOW YOU CAN EDIT

struct Solution;

impl Solution {
    #[allow(unused, clippy::needless_pass_by_value)]
    pub fn reverse_between(
        head: Option<Box<ListNode>>,
        left: i32,
        right: i32,
    ) -> Option<Box<ListNode>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_0() {
        let head = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode { val: 4, next: None })),
                })),
            })),
        }));

        let left = 1;
        let right = 2;

        let ans = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 3,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode { val: 4, next: None })),
                })),
            })),
        }));

        assert_eq!(Solution::reverse_between(head, left, right), ans);
    }
}
