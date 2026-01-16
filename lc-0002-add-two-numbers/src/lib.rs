// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    const fn new(val: i32) -> Self {
        Self { next: None, val }
    }
}

struct Solution;

impl Solution {
    #[allow(dead_code, clippy::needless_pass_by_value)]
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        // Transforms a sequence of ListNode into a Vec.
        let list_to_vec = |init_node: Option<Box<ListNode>>| -> Vec<i32> {
            let mut v: Vec<i32> = Vec::new();

            let mut previous = init_node;
            while let Some(boxed_node) = previous {
                v.push(boxed_node.val);
                previous = boxed_node.next;
            }

            v
        };

        let vec1 = list_to_vec(l1);
        let vec2 = list_to_vec(l2);

        let mut working_result: Vec<i32> = Vec::new();

        // Computes the result in Vec form.
        let mut overflow: i32 = 0;
        for i in 0.. {
            let (val1, val2) = (vec1.get(i), vec2.get(i));

            if val1.is_none() && val2.is_none() && overflow == 0 {
                break;
            }

            let sum = val1.unwrap_or(&0) + val2.unwrap_or(&0) + overflow;
            dbg!(sum % 10);

            overflow = i32::from(sum >= 10);

            working_result.push(sum % 10);
        }

        // Transform the result from Vec form to ListNode seq form.
        let mut working: Option<Box<ListNode>> = None;
        for digit in working_result.iter().rev() {
            let mut b = Box::new(ListNode::new(*digit));
            if let Some(w) = working {
                b.next = Some(w);
            }
            working = Some(b);
        }

        working
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // assert_eq!(Solution::add_two_numbers(Some(Box::new(ListNode::new(3))), Some(Box::new(ListNode::new(3)))), Some(Box::new(ListNode::new(6))));

        fn list_builder(vec: &[i32]) -> Option<Box<ListNode>> {
            let mut previous: Option<Box<ListNode>> = None;
            for val in vec.iter().rev() {
                let mut this = Box::new(ListNode::new(*val));
                if previous.is_some() {
                    this.next = previous;
                }
                previous = Some(this);
            }
            previous
        }

        assert_eq!(
            Solution::add_two_numbers(list_builder(&[2, 4, 3]), list_builder(&[5, 6, 4])),
            list_builder(&[7, 0, 8])
        );

        assert_eq!(
            Solution::add_two_numbers(
                list_builder(&[1, 9, 9, 9, 9, 9, 9, 9, 9, 9]),
                list_builder(&[9])
            ),
            list_builder(&[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1])
        );

        assert_eq!(
            Solution::add_two_numbers(
                list_builder(&[9, 9, 9, 9, 9, 9, 9]),
                list_builder(&[9, 9, 9, 9])
            ),
            list_builder(&[8, 9, 9, 9, 0, 0, 0, 1])
        );
    }
}
