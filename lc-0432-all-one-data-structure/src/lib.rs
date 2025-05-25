#![allow(dead_code)]

use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct AllOne {
    str_to_count: HashMap<String, u32>,
    count_to_str: HashMap<u32, HashSet<String>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl AllOne {
    fn new() -> Self {
        Self {
            str_to_count: HashMap::new(),
            count_to_str: HashMap::new(),
        }
    }

    fn inc(&mut self, key: String) {
        match self.str_to_count.entry(key.clone()) {
            Occupied(mut entry) => {
                let count: &mut u32 = entry.get_mut();

                // "move" key from count to (count + 1)
                self.count_to_str.get_mut(count).unwrap().remove(&key);
                self.count_to_str
                    .entry(*count + 1)
                    .and_modify(|v| {
                        v.insert(key.clone());
                    })
                    .or_insert_with(|| HashSet::from([key]));

                if self
                    .count_to_str
                    .get(count)
                    .is_some_and(std::collections::HashSet::is_empty)
                {
                    self.count_to_str.remove_entry(count);
                }

                // increment count
                *count += 1;
            }
            Vacant(entry) => {
                entry.insert(1);
                self.count_to_str
                    .entry(1)
                    .and_modify(|set| {
                        set.insert(key.clone());
                    })
                    .or_insert_with(|| HashSet::from([key]));
            }
        }
    }

    /// Decrements count associated with specified key. Removes the key if the
    /// new count is zero.
    ///
    /// The key is guaranteed to exist.
    fn dec(&mut self, key: String) {
        if self.str_to_count.get(&key).is_some_and(|v| *v == 1) {
            self.str_to_count.remove(&key);
            self.count_to_str.get_mut(&1).unwrap().remove(&key);

            if self
                .count_to_str
                .get(&1)
                .is_some_and(std::collections::HashSet::is_empty)
            {
                self.count_to_str.remove_entry(&1);
            }
        } else {
            let count = self.str_to_count.get_mut(&key).unwrap();

            // "move" key from count to (count - 1)
            self.count_to_str.get_mut(count).unwrap().remove(&key);
            self.count_to_str
                .entry(*count - 1)
                .and_modify(|v| {
                    v.insert(key.clone());
                })
                .or_insert_with(|| HashSet::from([key]));

            if self
                .count_to_str
                .get(count)
                .is_some_and(std::collections::HashSet::is_empty)
            {
                self.count_to_str.remove_entry(count);
            }

            *count -= 1;
        }
    }

    fn get_max_key(&self) -> String {
        if let Some(max_count) = self.count_to_str.keys().max() {
            return self
                .count_to_str
                .get(max_count)
                .unwrap()
                .iter()
                .nth(0)
                .unwrap()
                .clone();
        }

        String::new()
    }

    fn get_min_key(&self) -> String {
        if let Some(min_count) = self.count_to_str.keys().min() {
            return self
                .count_to_str
                .get(min_count)
                .unwrap()
                .iter()
                .nth(0)
                .unwrap()
                .clone();
        }

        String::new()
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
        let mut a = AllOne::new();
        a.inc("hello".to_string());
        a.inc("hello".to_string());
    }

    #[test]
    fn case_1() {
        let mut a = AllOne::new();

        a.inc("a".to_string());

        a.inc("b".to_string());
        a.inc("b".to_string());

        a.inc("c".to_string());
        a.inc("c".to_string());
        a.inc("c".to_string());
        dbg!(&a);

        a.dec("b".to_string());
        a.dec("b".to_string());
        dbg!(&a);

        assert_eq!(a.get_min_key(), "a".to_string());

        a.dec("a".to_string());
        dbg!(&a);

        assert_eq!(a.get_max_key(), "c".to_string());
        assert_eq!(a.get_min_key(), "c".to_string());
    }

    #[test]
    fn case_2() {
        let mut a = AllOne::new();

        a.inc("a".to_string());
        a.inc("b".to_string());
        a.inc("b".to_string());
        a.inc("b".to_string());
        a.inc("b".to_string());
        dbg!(&a);

        a.dec("b".to_string());
        dbg!(&a);
        a.dec("b".to_string());
        dbg!(&a);

        assert_eq!(a.get_max_key(), "b".to_string());
        assert_eq!(a.get_min_key(), "a".to_string());
    }

    #[test]
    fn case_3() {
        let mut a = AllOne::new();

        a.inc("hello".to_string());
        a.inc("hello".to_string());

        assert_eq!(a.get_max_key(), "hello".to_string());
        assert_eq!(a.get_min_key(), "hello".to_string());

        dbg!(&a);
        a.inc("leet".to_string());
        dbg!(&a);

        assert_eq!(a.get_max_key(), "hello".to_string());
        assert_eq!(a.get_min_key(), "leet".to_string());
    }

    #[test]
    fn case_4() {
        let mut a = AllOne::new();

        for _ in 0..10_000 {
            a.inc("a".to_string());
            a.inc("b".to_string());
        }

        for _ in 0..1_000 {
            a.inc("b".to_string());
        }

        dbg!(&a);

        assert_eq!(a.get_max_key(), "b".to_string());
        assert_eq!(a.get_min_key(), "a".to_string());
    }
}
