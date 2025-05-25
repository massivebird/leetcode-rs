#![allow(unused_variables, dead_code)]

struct Solution {}

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn add_spaces(s: String, spaces: Vec<i32>) -> String {
        let mut result: Vec<char> = Vec::new();

        let mut spaces_iter = spaces.into_iter().peekable();

        if spaces_iter.peek() == Some(&0) {
            result.push(' ');
            spaces_iter.next();
        }

        for (idx, c) in s.char_indices() {
            result.push(c);

            if spaces_iter.peek() == Some(&(i32::try_from(idx).unwrap() + 1)) {
                result.push(' ');
                spaces_iter.next();
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
