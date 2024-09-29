#![allow(dead_code)]

#[derive(Debug)]
struct MyCircularDeque {
    head_idx: usize,
    tail_idx: usize,
    size: usize,
    arr: Vec<i32>,
}

/*
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCircularDeque {
    fn new(k: i32) -> Self {
        Self {
            head_idx: 0,
            tail_idx: 0,
            size: 0,
            arr: vec![-1; (k + 1) as usize],
        }
    }

    fn insert_front(&mut self, value: i32) -> bool {
        if self.is_full() {
            return false;
        }

        if self.is_empty() {
            *self.arr.get_mut(self.head_idx).unwrap() = value;
            self.size += 1;
            return true;
        }

        self.head_idx = self.wrapping_decrement(self.head_idx);
        *self.arr.get_mut(self.head_idx).unwrap() = value;
        self.size += 1;

        true
    }

    fn insert_last(&mut self, value: i32) -> bool {
        if self.is_full() {
            return false;
        }

        if self.is_empty() {
            *self.arr.get_mut(self.tail_idx).unwrap() = value;
            self.size += 1;
            return true;
        }

        self.tail_idx = self.wrapping_increment(self.tail_idx);
        *self.arr.get_mut(self.tail_idx).unwrap() = value;
        self.size += 1;

        true
    }

    fn delete_front(&mut self) -> bool {
        if self.is_empty() {
            return false;
        }

        if self.size == 1 {
            *self.arr.get_mut(self.head_idx).unwrap() = -1;
            self.size -= 1;
            return true;
        }

        self.head_idx = self.wrapping_increment(self.head_idx);
        self.size -= 1;

        true
    }

    fn delete_last(&mut self) -> bool {
        if self.is_empty() {
            return false;
        }

        if self.size == 1 {
            *self.arr.get_mut(self.tail_idx).unwrap() = -1;
            self.size -= 1;
            return true;
        }

        self.tail_idx = self.wrapping_decrement(self.tail_idx);
        self.size -= 1;

        true
    }

    fn get_front(&self) -> i32 {
        *self.arr.get(self.head_idx).unwrap()
    }

    fn get_rear(&self) -> i32 {
        *self.arr.get(self.tail_idx).unwrap()
    }

    const fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn is_full(&self) -> bool {
        self.size == self.arr.capacity() - 1
    }

    fn wrapping_increment(&self, val: usize) -> usize {
        match val {
            other if other == self.arr.capacity() - 1 => 0,
            other => other + 1,
        }
    }

    fn wrapping_decrement(&self, val: usize) -> usize {
        match val {
            0 => self.arr.capacity() - 1,
            other => other - 1,
        }
    }
}

/*
 * Your MyCircularDeque object will be instantiated and called as such:
 * let obj = MyCircularDeque::new(k);
 * let ret_1: bool = obj.insert_front(value);
 * let ret_2: bool = obj.insert_last(value);
 * let ret_3: bool = obj.delete_front();
 * let ret_4: bool = obj.delete_last();
 * let ret_5: i32 = obj.get_front();
 * let ret_6: i32 = obj.get_rear();
 * let ret_7: bool = obj.is_empty();
 * let ret_8: bool = obj.is_full();
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let mut d = MyCircularDeque::new(3);

        assert!(d.insert_last(1));
        dbg!(&d);
        assert_eq!(d.get_front(), 1);

        assert!(d.insert_last(2));
        dbg!(&d);
        assert_eq!(d.get_front(), 1);
        assert_eq!(d.get_rear(), 2);

        assert!(d.insert_front(3));
        dbg!(&d);
        assert_eq!(d.get_front(), 3);
        assert_eq!(d.get_rear(), 2);

        assert!(!d.insert_front(4));
    }

    #[test]
    fn case_2() {
        let mut d = MyCircularDeque::new(8);

        assert!(d.insert_front(5));
        dbg!(&d);
        assert_eq!(d.get_front(), 5);
    }

    #[test]
    fn case_3() {
        let mut d = MyCircularDeque::new(4);

        assert!(d.insert_front(9));
        dbg!(&d);

        assert!(d.delete_last());
        dbg!(&d);
        assert_eq!(d.get_rear(), -1);
        assert_eq!(d.get_front(), -1);
    }

    #[test]
    fn case_4() {
        let mut d = MyCircularDeque::new(2);

        assert!(d.insert_front(7));
        dbg!(&d);

        assert!(d.delete_last());
        dbg!(&d);
        assert_eq!(d.get_front(), -1);

        assert!(d.insert_last(5));
        dbg!(&d);

        assert!(d.insert_front(0));
        dbg!(&d);

        assert_eq!(d.get_front(), 0);
        dbg!(&d);

        assert_eq!(d.get_rear(), 5);
        dbg!(&d);
    }
}
