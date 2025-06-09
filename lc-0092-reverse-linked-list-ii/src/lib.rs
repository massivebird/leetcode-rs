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
        // We'll convert this vector of values into a linked list at the end.
        let mut ans_vec: Vec<i32> = Vec::new();

        let mut head = head;

        let mut idx = 0;

        while let Some(val) = head.as_ref().map(|n| n.val) {
            // `left` and `right` are 1-based positions rather than
            // 0-based indices.
            if idx >= left - 1 && idx < right {
                ans_vec.insert(usize::try_from(left - 1).unwrap(), val);
            } else {
                ans_vec.push(val);
            }

            // Prepare next node
            head = head.unwrap().next;
            idx += 1;
        }

        // Convert vector to linked list.
        let mut ans: Option<Box<ListNode>> = None;

        for val in ans_vec.into_iter().rev() {
            ans = Some(Box::new(ListNode { val, next: ans }));
        }

        ans
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

        let left = 2;
        let right = 3;

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
