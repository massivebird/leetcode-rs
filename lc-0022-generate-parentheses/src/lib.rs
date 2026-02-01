//! Given n pairs of parentheses, write a function to generate all
//! combinations of well-formed parentheses (all pairs properly opened/closed).
//!
//! MB: basic backtracking approach!
//!
//! Last time I wrote a backtracking fn, I propagated their answers upwards in
//! vectors, continuously concatenating them. This time, I replaced that logic
//! with a single shared mutable reference that is pushed to whenever a string
//! is completed. This approach is way easier to conceptualize.

mod tests;

struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut ans: Vec<String> = Vec::new();

        Self::build(String::new(), &mut ans, usize::try_from(n).unwrap() * 2);

        ans
    }

    /// Recursively builds all possible and valid combinations of `n` pairs
    /// of parentheses.
    fn build(s: String, v: &mut Vec<String>, goal_len: usize) {
        // Push this string if it's completed!
        if s.len() == goal_len {
            v.push(s);
            return;
        }

        // Add openers `(` and closers `)` until their quotas are met.

        let openers = s.chars().filter(|c| *c == '(').count();

        if openers < goal_len / 2 {
            Self::build(s.clone() + "(", v, goal_len);
        }

        let closers = s.chars().filter(|c| *c == ')').count();

        if closers < openers {
            Self::build(s + ")", v, goal_len);
        }
    }
}
