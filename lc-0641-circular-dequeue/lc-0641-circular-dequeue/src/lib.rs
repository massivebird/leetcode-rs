struct MyCircularDeque {}

/*
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCircularDeque {
    fn new(k: i32) -> Self {
        todo!();
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
        todo!();
    }

    fn get_rear(&self) -> i32 {
        todo!();
    }

    fn is_empty(&self) -> bool {
        todo!();
    }

    fn is_full(&self) -> bool {
        todo!();
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
mod tests {}
