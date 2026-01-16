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
            arr: vec![-1; usize::try_from(k + 1).unwrap()],
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

    const fn is_full(&self) -> bool {
        self.size == self.arr.capacity() - 1
    }

    const fn wrapping_increment(&self, val: usize) -> usize {
        match val {
            other if other == self.arr.capacity() - 1 => 0,
            other => other + 1,
        }
    }

    const fn wrapping_decrement(&self, val: usize) -> usize {
        match val {
            0 => self.arr.capacity() - 1,
            other => other - 1,
        }
    }
}
