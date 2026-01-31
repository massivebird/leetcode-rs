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

#[test]
fn case_1() {
    let s = "catsandog".to_string();

    let word_dict = ["cats", "dog", "sand", "and", "cat"]
        .iter()
        .map(|&s| String::from(s))
        .collect::<Vec<String>>();

    assert!(!Solution::word_break(s, word_dict));
}

#[test]
fn case_2() {
    let s = "abcd".to_string();

    let word_dict = ["a", "abc", "b", "cd"]
        .iter()
        .map(|&s| String::from(s))
        .collect::<Vec<String>>();

    assert!(Solution::word_break(s, word_dict));
}

#[test]
fn case_3() {
    let s = "a".to_string();

    let word_dict = std::iter::once(&"b")
        .map(|&s| String::from(s))
        .collect::<Vec<String>>();

    assert!(!Solution::word_break(s, word_dict));
}

#[test]
fn case_4() {
    let s = "adcad".to_string();

    let word_dict = ["cbd", "ad"]
        .iter()
        .map(|&s| String::from(s))
        .collect::<Vec<String>>();

    assert!(!Solution::word_break(s, word_dict));
}

#[test]
fn case_5() {
    let s = "leetcode".to_string();

    let word_dict = ["leet", "etcode"]
        .iter()
        .map(|&s| String::from(s))
        .collect::<Vec<String>>();

    assert!(!Solution::word_break(s, word_dict));
}

#[test]
fn case_6() {
    let s = "cars".to_string();

    let word_dict = ["car", "ca", "rs"]
        .iter()
        .map(|&s| String::from(s))
        .collect::<Vec<String>>();

    assert!(Solution::word_break(s, word_dict));
}

#[test]
fn case_7() {
    let s = "a".to_string();

    let word_dict = ["a"]
        .iter()
        .map(|&s| String::from(s))
        .collect::<Vec<String>>();

    assert!(Solution::word_break(s, word_dict));
}
