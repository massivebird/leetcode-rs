use std::cell::RefCell;

struct CustomStack {
    head: Option<Box<RefCell<Node>>>,
    max_size: u32,
}

struct Node {
    val: u32,
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
            max_size,
        }
    }

    fn push(&self, x: i32) {
        todo!();
    }

    fn pop(&self) -> i32 {
        todo!();
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
