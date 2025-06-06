use std::collections::HashMap;

struct Solution;

#[allow(unused, clippy::needless_pass_by_value)]
impl Solution {
    // Input strings are equal in length.
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let mut map: HashMap<char, char> = HashMap::new();

        for (idx, src_char) in s.char_indices() {
            let target_char = t.chars().nth(idx).unwrap();

            match map.get_mut(&src_char) {
                Some(tc) if *tc != target_char => return false,
                Some(_) => (),
                None => {
                    // Mapping must be one-to-one.
                    if map.values().any(|c| *c == target_char) {
                        return false;
                    }

                    let _ = map.insert(src_char, target_char);
                }
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn case_0() {
        let s = "egg".to_string();
        let t = "add".to_string();

        assert!(Solution::is_isomorphic(s, t));
    }

    #[test]
    fn case_1() {
        let s = "foo".to_string();
        let t = "bar".to_string();

        assert!(!Solution::is_isomorphic(s, t));
    }

    #[test]
    fn case_2() {
        let s = "paper".to_string();
        let t = "title".to_string();

        assert!(Solution::is_isomorphic(s, t));
    }

    #[test]
    fn case_3() {
        let s = "badc".to_string();
        let t = "baba".to_string();

        assert!(!Solution::is_isomorphic(s, t));
    }
}
