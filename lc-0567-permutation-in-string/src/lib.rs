#![allow(dead_code)]

struct Solution;

impl Solution {
    /// Returns true is s2 contains a permutation of s1.
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        let char_to_idx = |c: char| -> usize { c as usize - 97 };

        let s1_frequencies = {
            let mut arr = [0u32; 26];

            for c in s1.chars() {
                *arr.get_mut(char_to_idx(c)).unwrap() += 1;
            }

            arr
        };
        let mut s2_frequencies = [0u32; 26];

        for (idx, s2_char) in s2.char_indices() {
            // increment this char's freq
            *s2_frequencies.get_mut(char_to_idx(s2_char)).unwrap() += 1;

            // decrement exiting char
            if idx >= s1.len() {
                let freq = *s2_frequencies.get_mut(idx - s1.len()).unwrap();
                *s2_frequencies.get_mut(idx - s1.len()).unwrap() = freq.saturating_sub(1);
            }

            dbg!(s2_frequencies);

            if s2_frequencies == s1_frequencies {
                return true;
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert!(Solution::check_inclusion(
            "ab".to_string(),
            "eidbaooo".to_string()
        ));
    }
}
