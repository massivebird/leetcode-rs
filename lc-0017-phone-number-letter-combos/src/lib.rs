//! Given a string containing digits from 2-9 inclusive, return all possible
//! letter combinations that the number could represent. Return the answer
//! in any order.
//!
//! MB: simple backtracking approach!

mod tests;

struct Solution;

impl Solution {
    #[allow(dead_code, clippy::needless_pass_by_value)]
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let mut ans: Vec<String> = Vec::new();

        Self::build(&digits, String::new(), &mut ans);

        ans
    }

    fn build(digits: &str, s: String, ans: &mut Vec<String>) {
        // Push this string if it's completed!
        if s.len() == digits.len() {
            ans.push(s);
            return;
        }

        let letters = match digits.chars().nth(s.len()).unwrap() {
            '2' => vec!['a', 'b', 'c'],
            '3' => vec!['d', 'e', 'f'],
            '4' => vec!['g', 'h', 'i'],
            '5' => vec!['j', 'k', 'l'],
            '6' => vec!['m', 'n', 'o'],
            '7' => vec!['p', 'q', 'r', 's'],
            '8' => vec!['t', 'u', 'v'],
            '9' => vec!['w', 'x', 'y', 'z'],
            _ => unreachable!(),
        };

        for c in &letters {
            Self::build(digits, format!("{s}{c}"), ans);
        }
    }
}
