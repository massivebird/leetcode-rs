#![cfg(test)]

use super::*;

#[test]
fn case_0() {
    let n = 3;
    let ans: Vec<String> = ["((()))", "(()())", "(())()", "()(())", "()()()"]
        .iter()
        .map(|&s| String::from(s))
        .collect();

    assert_eq!(Solution::generate_parenthesis(n), ans);
}

#[test]
fn case_1() {
    let n = 1;
    let ans: Vec<String> = vec!["()".to_owned()];

    assert_eq!(Solution::generate_parenthesis(n), ans);
}
