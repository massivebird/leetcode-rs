struct Solution;

impl Solution {
    // Both inputs consist only of lowercase letters.
    #[allow(dead_code, clippy::needless_pass_by_value)]
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut freq: [u32; 26] = [0; 26];

        let char_to_idx = |c: char| (c as u8 - 97) as usize;

        for c in magazine.chars() {
            freq[char_to_idx(c)] += 1;
        }

        for c in ransom_note.chars() {
            let Some(v) = freq.get_mut(char_to_idx(c)) else {
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
