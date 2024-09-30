#![allow(dead_code)]

// This is my singly linked list approach!

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct CustomStack {
    head: Option<Rc<RefCell<Node>>>,
    size: u32,
    max_size: u32,
}

#[derive(Debug)]
struct Node {
    val: i32,
    next: Option<Rc<RefCell<Node>>>,
}

impl CustomStack {
    const fn new(max_size: i32) -> Self {
        Self {
            head: None,
            size: 0,
            max_size: max_size as u32,
        }
    }

    /// Adds x to the top of the stack if the stack is not at max size.
    fn push(&mut self, x: i32) {
        if self.size == self.max_size {
            return;
        }

        let new_head = Rc::new(RefCell::new(Node { val: x, next: None }));

        // Link the new head to the previous head, if one exists.
        if let Some(old_head) = self.head.take() {
            new_head.borrow_mut().next = Some(old_head);
        }

        self.head = Some(new_head);

        self.size += 1;
    }

    /// Pops and returns the top of the stack, or -1 if stack is empty.
    fn pop(&mut self) -> i32 {
        if self.size == 0 {
            return -1;
        }

        let old_head = self.head.take().unwrap();

        let new_head = old_head.borrow_mut().next.take();
        self.head = new_head;

        self.size -= 1;

        // I create an owned binding of the return value because Rust otherwise
        // throws `error[E0597]: [old_head] does not live long enough.`
        //
        // Even though val is i32 (impl Copy), I guess we would be returning
        // it as a field of an old_head borrow, which looks bad to the borrow
        // checker.
        //
        // The binding solves this by (more explicitly) performing a copy before
        // old_head is invalidated.
        let val = old_head.borrow().val;
        val
    }

    /// Increment the bottom k elements of the stack by val.
    ///
    /// If k > size, increment all items in the stack.
    fn increment(&self, k: i32, val: i32) {
        if self.size == 0 {
            return;
        }

        let num_nodes_to_skip = self.size.saturating_sub(k as u32);

        // De facto iterator. We'll mutate this as we go.
        let mut current_node = Rc::clone(self.head.as_ref().unwrap());

        // Moves the iterator to the next node. Just an abstraction for
        // code reuse.
        macro_rules! move_current_node {
            () => {
                let next = Rc::clone(&current_node.as_ref().borrow().next.clone().unwrap());
                current_node = next;
            };
        }

        // Eat the nodes we're meant to skip. Do not mutate them.
        for _ in 0..num_nodes_to_skip {
            move_current_node!();
        }

        let num_nodes_to_mutate = self.size - num_nodes_to_skip;

        // To avoid iterating OOB, we'll mutate the current node _before_
        // moving to the next one.

        // Preliminary mutation.
        current_node.borrow_mut().val += val;

        // Begin range at 1 since we already performed one mutation.
        for _ in 1..num_nodes_to_mutate {
            move_current_node!();
            current_node.borrow_mut().val += val;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let mut s = CustomStack::new(3);

        s.push(1);
        s.push(2);

        assert_eq!(s.pop(), 2);

        s.push(2);
        s.push(3);
        s.push(4);

        s.increment(5, 100);
        s.increment(2, 100);

        assert_eq!(s.pop(), 103);
        assert_eq!(s.pop(), 202);
        assert_eq!(s.pop(), 201);
        assert_eq!(s.pop(), -1);
    }
}
