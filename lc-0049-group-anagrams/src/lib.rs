struct Solution;

#[allow(unused, clippy::needless_pass_by_value)]
impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        use std::collections::HashMap;

        let mut anagrams: HashMap<[u32; 26], Vec<String>> = HashMap::new();

        for s in strs {
            let mut freqs = [0u32; 26];

            for c in s.chars() {
                freqs[(c as u8 - 97) as usize] += 1;
            }

            anagrams
                .entry(freqs)
                .and_modify(|v| v.push(s.clone()))
                .or_insert_with(|| vec![s]);
        }

        let mut res = Vec::new();

        for a in anagrams.into_values() {
            res.push(a);
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::string::ToString;

    #[test]
    fn case_0() {
        let strs = ["eat", "tea", "tan", "ate", "nat", "bat"]
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<String>>();

        let ans = vec![vec!["bat"], vec!["nat", "tan"], vec!["ate", "eat", "tea"]];

        assert_eq!(Solution::group_anagrams(strs), ans);
    }
}
