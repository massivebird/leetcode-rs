#![allow(unused_variables, dead_code)]

struct Solution {}

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn add_spaces(s: String, spaces: Vec<i32>) -> String {
        let mut result: Vec<char> = Vec::new();

        if spaces.contains(&0) {
            result.push(' ');
        }

        for (idx, c) in s.char_indices() {
            result.push(c);

            if spaces.contains(&((idx + 1) as i32)) {
                result.push(' ');
            }
        }

        result.iter().collect::<String>()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn case_1() {
        assert_eq!(
            "Leetcode Helps Me Learn".to_string(),
            Solution::add_spaces("LeetcodeHelpsMeLearn".to_string(), vec![8, 13, 15])
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(
            "i code in py thon".to_string(),
            Solution::add_spaces("icodeinpython".to_string(), vec![1, 5, 7, 9])
        );
    }

    #[test]
    fn case_3() {
        assert_eq!(
            " s p a c i n g".to_string(),
            Solution::add_spaces("spacing".to_string(), vec![0, 1, 2, 3, 4, 5, 6])
        );
    }
}
