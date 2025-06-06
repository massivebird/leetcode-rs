struct Solution;

#[allow(unused, clippy::needless_pass_by_value)]
impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_0() {
        let ransom_note = "a".to_string();
        let magazine = "b".to_string();

        assert!(!Solution::can_construct(ransom_note, magazine));
    }

    #[test]
    fn case_1() {
        let ransom_note = "aa".to_string();
        let magazine = "ab".to_string();

        assert!(Solution::can_construct(ransom_note, magazine));
    }
}
