use std::collections::HashMap;
use rand::Rng;

struct RandomizedSet {
    exists: HashMap<i32, bool>,
    vals: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
#[allow(dead_code, clippy::missing_const_for_fn)]
impl RandomizedSet {
    fn new() -> Self {
        Self {
            exists: HashMap::new(),
            vals: Vec::new(),
        }
    }

    fn insert(&mut self, val: i32) -> bool {
        if self.exists.get(&val).is_some_and(|b| *b) {
            return false;
        }

        self.exists.insert(val, true);
        self.vals.push(val);

        true
    }

    fn remove(&mut self, val: i32) -> bool {
        if self.exists.get(&val).is_some_and(|b| *b) {
            *self.exists.get_mut(&val).unwrap() = false;

            let pos = self.vals.iter().position(|v| *v == val).unwrap();

            self.vals.remove(pos);

            return true;
        }

        false
    }

    fn get_random(&self) -> i32 {
        let mut rng = rand::thread_rng();
        let idx = rng.gen_range(0..self.vals.len());
        self.vals[idx]
    }
}

// Your RandomizedSet object will be instantiated and called as such:
// let obj = RandomizedSet::new();
// let ret_1: bool = obj.insert(val);
// let ret_2: bool = obj.remove(val);
// let ret_3: i32 = obj.get_random();
