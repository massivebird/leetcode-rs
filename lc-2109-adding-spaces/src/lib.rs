#![allow(unused_variables, dead_code)]

struct Solution {}

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn add_spaces(s: String, spaces: Vec<i32>) -> String {
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(
            "Leetcode Helps Me Learn".to_string(),
            Solution::add_spaces("LeetcodeHelpsMeLearn".to_string(), vec![8, 13, 15])
        );
    }
}
