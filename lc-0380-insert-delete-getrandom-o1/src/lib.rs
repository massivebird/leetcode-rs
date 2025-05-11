use rand::Rng;
use std::collections::HashSet;

#[derive(Default)]
struct RandomizedSet {
    hash_set: HashSet<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
#[allow(dead_code, clippy::missing_const_for_fn)]
impl RandomizedSet {
    fn new() -> Self {
        Self::default()
    }

    fn insert(&mut self, val: i32) -> bool {
        if self.hash_set.contains(&val) {
            return false;
        }

        self.hash_set.insert(val);

        true
    }

    fn remove(&mut self, val: i32) -> bool {
        if self.hash_set.contains(&val) {
            self.hash_set.remove(&val);
            return true;
        }

        false
    }

    fn get_random(&self) -> i32 {
        let mut rng = rand::thread_rng();
        let idx = rng.gen_range(0..self.hash_set.len());
        *self.hash_set.iter().nth(idx).unwrap()
    }
}

// Your RandomizedSet object will be instantiated and called as such:
// let obj = RandomizedSet::new();
// let ret_1: bool = obj.insert(val);
// let ret_2: bool = obj.remove(val);
// let ret_3: i32 = obj.get_random();
