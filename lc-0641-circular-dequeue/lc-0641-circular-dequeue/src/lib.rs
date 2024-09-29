struct MyCircularDeque {
    head_idx: usize,
    tail_idx: usize,
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
            arr: Vec::with_capacity(k as usize),
        }
    }

    fn insert_front(&self, value: i32) -> bool {
        todo!();
    }

    fn insert_last(&self, value: i32) -> bool {
        todo!();
    }

    fn delete_front(&self) -> bool {
        todo!();
    }

    fn delete_last(&self) -> bool {
        todo!();
    }

    fn get_front(&self) -> i32 {
        *self.arr.get(self.head_idx).unwrap()
    }

    fn get_rear(&self) -> i32 {
        *self.arr.get(self.tail_idx).unwrap()
    }

    fn is_empty(&self) -> bool {
        self.head_idx == self.tail_idx
    }

    fn is_full(&self) -> bool {
        todo!();
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
    fn wrapping_stuff() {
        let mut d = MyCircularDeque::new(3);

        d.head_idx = d.wrapping_increment(d.head_idx);
        assert_eq!(d.head_idx, 1);

        d.head_idx = d.wrapping_increment(d.head_idx);
        assert_eq!(d.head_idx, 2);

        d.head_idx = d.wrapping_increment(d.head_idx);
        assert_eq!(d.head_idx, 0);

        d.head_idx = d.wrapping_decrement(d.head_idx);
        assert_eq!(d.head_idx, 2);

        d.head_idx = d.wrapping_decrement(d.head_idx);
        assert_eq!(d.head_idx, 1);
    }
}
