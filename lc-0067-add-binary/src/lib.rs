struct Solution;

#[allow(unused, clippy::needless_pass_by_value)]
impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let mut ans = String::new();

        let mut carry = false;

        // Assign the longer of the two input strings to `a`, and the shorter
        // to `b`. Simplifies some logic.
        let (a, b) = if a.len() >= b.len() { (a, b) } else { (b, a) };

        for (idx, a_char) in a.chars().rev().enumerate() {
            if let Some(b_char) = b.chars().rev().nth(idx) {
                match (a_char, b_char, carry) {
                    ('1', '1', false) => {
                        ans.push('0');
                        carry = true;
                    }
                    ('1', '0', true) | ('0', '1', true) => {
                        ans.push('0');
                    }
                    ('1', '0', false) | ('0', '1', false) | ('1', '1', true) => {
                        ans.push('1');
                    }
                    ('0', '0', true) => {
                        ans.push('1');
                        carry = false;
                    }
                    _ => ans.push('0'),
                }
                continue;
            }

            // `b` string has been exhausted. Process remaining bits in `a`.
            match (a_char, carry) {
                ('1', true) => {
                    ans.push('0');
                }
                ('1', false) => ans.push('1'),
                ('0', true) => {
                    ans.push('1');
                    carry = false;
                }
                _ => ans.push('0'),
            }
        }

        if carry {
            ans.push('1');
        }

        ans.chars().rev().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_0() {
        let a = "11".to_owned();
        let b = "1".to_owned();
        let ans = "100".to_owned();

        assert_eq!(Solution::add_binary(a, b), ans);
    }

    #[test]
    fn case_1() {
        let a = "1010".to_owned();
        let b = "1011".to_owned();
        let ans = "10101".to_owned();

        assert_eq!(Solution::add_binary(a, b), ans);
    }
}
