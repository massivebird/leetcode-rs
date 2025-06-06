use std::collections::HashMap;

struct Solution;

#[allow(unused, clippy::needless_pass_by_value)]
impl Solution {
    // Both inputs consist only of lowercase letters.
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut freq: HashMap<char, u32> = HashMap::new();

        for c in magazine.chars() {
            freq.entry(c).and_modify(|v| *v += 1).or_insert(1);
        }

        for c in ransom_note.chars() {
            let Some(v) = freq.get_mut(&c) else {
                // This char isn't in the magazine at all.
                return false;
            };

            if *v == 0 {
                return false;
            }

            *v -= 1;
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_0() {
        let ransom_note = "a".to_string();
        let magazine = "b".to_string();

        assert!(!Solution::can_construct(ransom_note, magazine));
    }

    #[test]
    fn case_1() {
        let ransom_note = "aa".to_string();
        let magazine = "ab".to_string();

        assert!(!Solution::can_construct(ransom_note, magazine));
    }
}
