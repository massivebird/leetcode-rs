struct Solution;

#[allow(unused, clippy::needless_pass_by_value)]
impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let Some((last_space_pos, _)) = s.trim().char_indices().rfind(|(_, c)| c.is_whitespace())
        else {
            return i32::try_from(s.trim().len()).unwrap();
        };

        let length = s
            .trim()
            .chars()
            .skip(last_space_pos + 1)
            .take_while(|c| !c.is_whitespace())
            .count();

        i32::try_from(length).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_0() {
        assert_eq!(Solution::length_of_last_word("Hello World".to_string()), 5);
    }

    #[test]
    fn case_1() {
        assert_eq!(
            Solution::length_of_last_word("   fly me   to   the moon  ".to_string()),
            4
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::length_of_last_word(" a".to_string()), 1);
    }
}
