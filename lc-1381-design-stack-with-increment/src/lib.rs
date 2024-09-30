#![allow(dead_code)]

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

/*
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl CustomStack {
    const fn new(max_size: i32) -> Self {
        Self {
            head: None,
            size: 0,
            max_size: max_size as u32,
        }
    }

    // Adds x to the top of the stack if the stack is not at max size.
    fn push(&mut self, x: i32) {
        if self.size == self.max_size {
            return;
        }

        let new_head = Rc::new(RefCell::new(Node { val: x, next: None }));

        if let Some(old_head) = self.head.take() {
            new_head.borrow_mut().next = Some(old_head);
        }

        self.head = Some(new_head);

        self.size += 1;
    }

    // Pops and returns the top of the stack, or -1 if stack is empty.
    fn pop(&mut self) -> i32 {
        if self.size == 0 {
            return -1;
        }

        let old_head = self.head.take().unwrap();

        let new_head = old_head.borrow_mut().next.take();
        self.head = new_head;

        self.size -= 1;

        let val = old_head.borrow().val;
        val
    }

    // Increment the bottom k elements of the stack by val.
    //
    // If k > size, increment all items in the stack.
    fn increment(&self, k: i32, val: i32) {
        if self.size == 0 {
            return;
        }

        let num_skips = self.size.saturating_sub(k as u32);
        dbg!(num_skips);

        let mut current_head = Rc::clone(self.head.as_ref().unwrap());
        dbg!(&current_head);

        for _ in 0..num_skips {
            let next = Rc::clone(&current_head.as_ref().borrow().next.clone().unwrap());
            current_head = next;
        }

        let num_to_inc = self.size - num_skips;
        dbg!(num_to_inc);

        current_head.borrow_mut().val += val;
        dbg!(&current_head);

        for _ in 1..num_to_inc {
            // go next
            let next = Rc::clone(&current_head.as_ref().borrow().next.clone().unwrap());
            current_head = next;

            current_head.borrow_mut().val += val;
            dbg!(&current_head);
        }
    }
}

/*
 * Your CustomStack object will be instantiated and called as such:
 * let obj = CustomStack::new(maxSize);
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * obj.increment(k, val);
 */

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
