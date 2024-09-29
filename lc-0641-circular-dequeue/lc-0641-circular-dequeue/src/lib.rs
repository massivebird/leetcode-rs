#![allow(dead_code)]

use std::cell::RefCell;
use std::rc::{Rc, Weak};

mod solution_1;

#[derive(Debug)]
struct MyCircularDeque {
    head: Option<Rc<RefCell<Node>>>,
    tail: Option<Rc<RefCell<Node>>>,
    len: usize,
    capacity: usize,
}

#[derive(Debug)]
struct Node {
    elem: i32,
    next: Option<Rc<RefCell<Node>>>,
    // Weak references prevent reference cycles!
    // Appears to reduce space complexity slightly.
    prev: Option<Weak<RefCell<Node>>>,
}

/*
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCircularDeque {
    const fn new(k: i32) -> Self {
        Self {
            head: None,
            tail: None,
            len: 0,
            capacity: k as usize,
        }
    }

    fn insert_front(&mut self, value: i32) -> bool {
        if self.is_full() {
            return false;
        }

        let head = self.head.take();

        let new_head = Rc::new(RefCell::new(Node {
            elem: value,
            next: None,
            prev: None,
        }));

        match head {
            None => self.tail = Some(Rc::clone(&new_head)),
            Some(old_head) => {
                new_head.borrow_mut().next = Some(Rc::clone(&old_head));
                old_head.borrow_mut().prev = Some(Rc::downgrade(&new_head));
            }
        }

        self.head = Some(new_head);
        self.len += 1;

        true
    }

    fn insert_last(&mut self, value: i32) -> bool {
        if self.is_full() {
            return false;
        }

        let new_tail = Rc::new(RefCell::new(Node {
            elem: value,
            next: None,
            prev: None,
        }));

        match self.tail.take() {
            None => {
                self.head = Some(Rc::clone(&new_tail));
            }
            Some(old_tail) => {
                new_tail.borrow_mut().prev = Some(Rc::downgrade(&old_tail));
                old_tail.borrow_mut().next = Some(Rc::clone(&new_tail));
            }
        }

        self.tail = Some(new_tail);
        self.len += 1;

        true
    }

    fn delete_front(&mut self) -> bool {
        if self.is_empty() {
            return false;
        }

        match self.head.take() {
            None => unreachable!(),
            Some(old_head) => {
                if let Some(new_head) = old_head.borrow_mut().next.take() {
                    new_head.borrow_mut().prev = None;
                    self.head = Some(new_head);
                } else {
                    self.tail = None;
                }
            }
        }

        self.len -= 1;

        true
    }

    fn delete_last(&mut self) -> bool {
        if self.is_empty() {
            return false;
        }

        match self.tail.take() {
            None => unreachable!(),
            Some(old_tail) => {
                if let Some(new_tail) = old_tail.borrow_mut().prev.take() {
                    new_tail.upgrade().unwrap().borrow_mut().next = None;
                    self.tail = Some(new_tail.upgrade().unwrap());
                } else {
                    self.head = None;
                }
            }
        }

        self.len -= 1;

        true
    }

    fn get_front(&self) -> i32 {
        self.head.as_ref().map_or(-1, |node| node.borrow().elem)
    }

    fn get_rear(&self) -> i32 {
        self.tail.as_ref().map_or(-1, |node| node.borrow().elem)
    }

    const fn is_empty(&self) -> bool {
        self.len == 0
    }

    fn is_full(&self) -> bool {
        self.len == self.capacity
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let mut d = MyCircularDeque::new(3);

        assert!(d.insert_last(1));
        assert_eq!(d.get_front(), 1);

        assert!(d.insert_last(2));
        assert_eq!(d.get_front(), 1);
        assert_eq!(d.get_rear(), 2);

        assert!(d.insert_front(3));
        assert_eq!(d.get_front(), 3);
        assert_eq!(d.get_rear(), 2);

        assert!(!d.insert_front(4));
    }

    #[test]
    fn case_2() {
        let mut d = MyCircularDeque::new(8);

        assert!(d.insert_front(5));
        assert_eq!(d.get_front(), 5);
    }

    #[test]
    fn case_3() {
        let mut d = MyCircularDeque::new(4);

        assert!(d.insert_front(9));

        assert!(d.delete_last());
        assert_eq!(d.get_rear(), -1);
        assert_eq!(d.get_front(), -1);
    }

    #[test]
    fn case_4() {
        let mut d = MyCircularDeque::new(2);

        assert!(d.insert_front(7));

        assert!(d.delete_last());
        assert_eq!(d.get_front(), -1);

        assert!(d.insert_last(5));

        assert!(d.insert_front(0));

        assert_eq!(d.get_front(), 0);

        assert_eq!(d.get_rear(), 5);
    }
}
