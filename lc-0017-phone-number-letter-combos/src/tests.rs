#![cfg(test)]

use super::*;

#[test]
fn case_0() {
    let digits = "23".to_string();
    let ans: Vec<String> = ["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]
        .iter()
        .map(|&s| String::from(s))
        .collect();

    assert_eq!(Solution::letter_combinations(digits), ans);
}
