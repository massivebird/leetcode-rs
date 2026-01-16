#![allow(dead_code)]

struct Solution;

impl Solution {
    /// Returns true is s2 contains a permutation of s1.
    #[allow(clippy::needless_pass_by_value)]
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        let char_to_idx = |c: char| -> usize { c as usize - 97 };

        // These arrays represent the frequency of each character.
        let s1_freqs = {
            let mut arr = [0i32; 26];

            for c in s1.chars() {
                *arr.get_mut(char_to_idx(c)).unwrap() += 1;
            }

            arr
        };

        // This frequencies array will be mutated so as to represent an
        // s1-length-long-window of characters in s2.
        let mut s2_freqs = [0i32; 26];

        let mut num_matching_freqs = 0;

        for char in s2.chars().take(s1.len()) {
            *s2_freqs.get_mut(char_to_idx(char)).unwrap() += 1;
        }

        for idx in 0..26 {
            if s1_freqs.get(idx).unwrap() == s2_freqs.get(idx).unwrap() {
                num_matching_freqs += 1;
            }
        }

        for (idx, char) in s2.char_indices().skip(s1.len()) {
            // Checks answer for initial matches AND per loop iteration
            if num_matching_freqs == 26 {
                return true;
            }

            // Increment current char's frequency.
            let right_freq_idx = char_to_idx(char);
            *s2_freqs.get_mut(right_freq_idx).unwrap() += 1;

            // Mutate matching freq count after incrementing if char freq either:
            // 1) Now matches after incrementing, or
            // 2) Is now no longer matching.
            if s2_freqs.get(right_freq_idx).unwrap() == s1_freqs.get(right_freq_idx).unwrap() {
                num_matching_freqs += 1;
            } else if *s2_freqs.get(right_freq_idx).unwrap()
                == s1_freqs.get(right_freq_idx).unwrap() + 1
            {
                num_matching_freqs -= 1;
            }

            // Decrement the frequency of the character exiting the window.
            let left_freq_idx = char_to_idx(s2.chars().nth(idx - s1.len()).unwrap());
            *s2_freqs.get_mut(left_freq_idx).unwrap() -= 1;

            // Mutate matching freq count after decrementing if char freq either:
            // 1) Now matches after decrementing, or
            // 2) Is now no longer matching.
            if s2_freqs.get(left_freq_idx).unwrap() == s1_freqs.get(left_freq_idx).unwrap() {
                num_matching_freqs += 1;
            } else if *s2_freqs.get(left_freq_idx).unwrap()
                == s1_freqs.get(left_freq_idx).unwrap() - 1
            {
                num_matching_freqs -= 1;
            }
        }

        // Covers final loop iteration
        num_matching_freqs == 26
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

    #[test]
    fn case_2() {
        assert!(Solution::check_inclusion(
            "abc".to_string(),
            "bbbca".to_string()
        ));
    }

    #[test]
    fn case_3() {
        assert!(!Solution::check_inclusion(
            "ab".to_string(),
            "eidboaoo".to_string()
        ));
    }

    #[test]
    fn case_4() {
        assert!(Solution::check_inclusion("a".to_string(), "ab".to_string()));
    }
}
