struct Solution;

#[allow(unused, clippy::needless_pass_by_value)]
impl Solution {
    pub fn is_valid(s: String) -> bool {
        use std::collections::VecDeque;
        let mut stack: VecDeque<char> = VecDeque::new();

        for c in s.chars() {
            if matches!(c, '(' | '{' | '[') {
                stack.push_back(c);
                continue;
            }

            let Some(last) = stack.iter().last() else {
                return false;
            };

            match (last, c) {
                ('(', ')') | ('{', '}') | ('[', ']') => {
                    stack.pop_back();
                }
                _ => return false,
            }
        }

        stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_0() {
        let s = "()".into();

        assert!(Solution::is_valid(s));
    }

    #[test]
    fn case_1() {
        let s = "(()".into();

        assert!(!Solution::is_valid(s));
    }
}
