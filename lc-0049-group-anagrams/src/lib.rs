struct Solution;

impl Solution {
    #[allow(dead_code, clippy::needless_pass_by_value)]
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        use std::collections::HashMap;

        let mut anagrams: HashMap<[u32; 26], Vec<String>> = HashMap::new();

        for s in strs {
            let mut freqs = [0u32; 26];

            for c in s.bytes() {
                freqs[(c - b'a') as usize] += 1;
            }

            anagrams
                .entry(freqs)
                .and_modify(|v| v.push(s.clone()))
                .or_insert_with(|| vec![s]);
        }

        anagrams.into_values().collect()
    }
}

// Writing tests for this challenge is totally uncool. The answer can be
// returned in an arbitrary order, and the needless pass by value makes copying
// the inputs/outputs annoying.
