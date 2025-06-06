struct Solution;

#[allow(unused, clippy::needless_pass_by_value)]
impl Solution {
    // Inputs consist only of lowercase English letters.
    pub fn is_anagram(s: String, t: String) -> bool {
        use std::collections::HashMap;

        // Use signed integers to avoid checking for underflow.
        let mut freqs = [0i32; 26];

        let char_to_idx = |c: char| (c as u8 - 97) as usize;

        for c in s.chars() {
            freqs[char_to_idx(c)] += 1;
        }

        for c in t.chars() {
            unsafe {
                *freqs.get_unchecked_mut(char_to_idx(c)) -= 1;
            }
        }

        freqs == [0; 26]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_0() {
        let s = "anagram".to_string();
        let t = "aangram".to_string();

        assert!(Solution::is_anagram(s, t));
    }

    #[test]
    fn case_1() {
        let s = "rat".to_string();
        let t = "car".to_string();

        assert!(!Solution::is_anagram(s, t));
    }

    #[test]
    fn case_2() {
        let s = "cat".to_string();
        let t = "dog".to_string();

        assert!(!Solution::is_anagram(s, t));
    }
}
