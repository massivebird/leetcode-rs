struct Solution;

#[allow(unused, clippy::needless_pass_by_value)]
impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let mut s_chars = s.chars();
        let mut t_chars = t.chars();

        'outer: loop {
            let Some(s_next) = s_chars.next() else {
                break;
            };

            loop {
                match t_chars.next() {
                    Some(c) if c == s_next => continue 'outer,
                    Some(_) => (),
                    None => return false,
                }
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_0() {
        let s = "abc".to_owned();
        let t = "ahbgdc".to_owned();

        assert!(Solution::is_subsequence(s, t));
    }

    #[test]
    fn case_1() {
        let s = "axc".to_owned();
        let t = "ahbgdc".to_owned();

        assert!(!Solution::is_subsequence(s, t));
    }
}
