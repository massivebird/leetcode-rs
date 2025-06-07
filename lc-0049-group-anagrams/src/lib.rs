struct Solution;

#[allow(unused, clippy::needless_pass_by_value)]
impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        todo!()
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
