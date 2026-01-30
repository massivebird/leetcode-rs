#![cfg(test)]

use super::*;

#[test]
fn case_0() {
    let s = "leetcode".to_string();

    let word_dict = ["leet", "code"]
        .iter()
        .map(|&s| String::from(s))
        .collect::<Vec<String>>();

    assert!(Solution::word_break(s, word_dict));
}
