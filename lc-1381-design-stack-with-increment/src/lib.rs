use std::cell::RefCell;

struct CustomStack {
    head: Option<Box<RefCell<Node>>>,
    size: u32,
    max_size: u32,
}

struct Node {
    val: i32,
    next: Option<Box<RefCell<Node>>>,
}

/*
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl CustomStack {
    const fn new(max_size: u32) -> Self {
        Self {
            head: None,
            size: 0,
            max_size,
        }
    }

    // Adds x to the top of the stack if the stack is not at max size.
    fn push(&mut self, x: i32) {
        if self.size == self.max_size {
            return;
        }

        let new_head = Box::new(RefCell::new(Node { val: x, next: None }));

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

        self.head = old_head.borrow_mut().next.take();

        self.size -= 1;
    }

    fn increment(&self, k: i32, val: i32) {
        todo!();
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
    fn it_works() {
        let s = CustomStack::new(2);
    }
}
