use std::collections::HashMap;

struct Solution;

#[allow(unused, clippy::needless_pass_by_value)]
impl Solution {
    // Input strings are equal in length.
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let mut src_target: HashMap<char, char> = HashMap::new();
        let mut target_src: HashMap<char, char> = HashMap::new();

        for (idx, src_char) in s.char_indices() {
            let target_char = t.chars().nth(idx).unwrap();

            // Check if src is mapped to a different target.
            let src_already_mapped = src_target
                .insert(src_char, target_char)
                .is_some_and(|t| t != target_char);

            // Mapping must be one-to-one; all src chars must map to
            // unique targets.
            let target_is_taken = target_src
                .insert(target_char, src_char)
                .is_some_and(|s| s != src_char);

            if src_already_mapped || target_is_taken {
                return false;
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
