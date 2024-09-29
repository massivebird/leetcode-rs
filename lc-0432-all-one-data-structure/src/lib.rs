use std::collections::HashMap;

struct AllOne {
    keys: HashMap<String, u32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl AllOne {
    fn new() -> Self {
        Self {
            keys: HashMap::new(),
        }
    }

    fn inc(&mut self, key: String) {
        self.keys.entry(key).and_modify(|v| *v += 1).or_insert(1);
    }

    /// Decrements count associated with specified key. Removes the key if the
    /// new count is zero.
    ///
    /// The key is guaranteed to exist.
    fn dec(&mut self, key: String) {
        if *self.keys.get(&key).unwrap() == 1 {
            self.keys.remove(&key);
        } else {
            self.keys.entry(key).and_modify(|v| *v -= 1);
        }
    }

    fn get_max_key(&self) -> String {
        todo!();
    }

    fn get_min_key(&self) -> String {
        todo!();
    }
}

/*
 * Your AllOne object will be instantiated and called as such:
 * let obj = AllOne::new();
 * obj.inc(key);
 * obj.dec(key);
 * let ret_3: String = obj.get_max_key();
 * let ret_4: String = obj.get_min_key();
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let a = AllOne::new();
    }
}
