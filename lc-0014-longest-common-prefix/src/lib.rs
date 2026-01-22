struct Solution;

impl Solution {
    #[allow(dead_code, clippy::needless_pass_by_value)]
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut buf = String::new();

        // You gave me an empty vector? Are you for serious?
        let Some(first_word) = strs.first() else {
            return buf;
        };

        for (i, c) in first_word.char_indices() {
            if strs
                .iter()
                .all(|w| w.chars().nth(i).is_some_and(|o| c == o))
            {
                buf.push(c);
            } else {
                break;
            }
        }

        buf
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_0() {
        let strs = vec![
            "flower".to_string(),
            "flow".to_string(),
            "flight".to_string(),
        ];

        assert_eq!(Solution::longest_common_prefix(strs), "fl".to_string());
    }

    #[test]
    fn case_1() {
        let strs = vec![
            "racecar".to_string(),
            "dog".to_string(),
            "clint".to_string(),
        ];

        assert_eq!(Solution::longest_common_prefix(strs), String::new());
    }
}
