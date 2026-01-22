// --- PROVIDED, DO NOT EDIT

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    #[allow(dead_code, clippy::missing_const_for_fn, clippy::use_self)]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

// --- OK NOW YOU CAN EDIT

struct Solution;

impl Solution {
    #[allow(dead_code, clippy::needless_pass_by_value)]
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        // I create the final linked list answer out of a vector containing
        // the combined, sorted values from both lists.
        let mut ans_vec: Vec<i32> = Vec::new();

        let mut node1 = list1;
        let mut node2 = list2;

        while let Some(val1) = node1.as_ref().map(|n| n.val) {
            while let Some(val2) = node2.as_ref().map(|n| n.val) {
                // Push to the vector any lesser values from list2.
                if val2 <= val1 {
                    ans_vec.push(val2);
                    node2 = node2.unwrap().next;
                } else {
                    break;
                }
            }

            ans_vec.push(val1);

            node1 = node1.unwrap().next;
        }

        // Push any remaining list2 values.
        while let Some(val2) = node2.as_ref().map(|n| n.val) {
            ans_vec.push(val2);
            node2 = node2.unwrap().next;
        }

        // Now we're ready to build the final linked list!
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
        let list1 = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode { val: 3, next: None })),
        }));

        let list2 = Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode { val: 4, next: None })),
        }));

        let ans = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode { val: 4, next: None })),
                })),
            })),
        }));

        assert_eq!(Solution::merge_two_lists(list1, list2), ans);
    }
}
