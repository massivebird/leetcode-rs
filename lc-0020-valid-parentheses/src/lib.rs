struct Solution;

#[allow(unused, clippy::needless_pass_by_value)]
impl Solution {
    pub fn is_valid(s: String) -> bool {
        use std::collections::VecDeque;
        let mut stack: VecDeque<u8> = VecDeque::new();

        for c in s.as_bytes() {
            if matches!(c, b'(' | b'{' | b'[') {
                stack.push_back(*c);
                continue;
            }

            let Some(last) = stack.iter().last() else {
                return false;
            };

            match (last, c) {
                (b'(', b')') | (b'{', b'}') | (b'[', b']') => {
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
