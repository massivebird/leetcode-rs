struct Solution;

#[allow(unused, clippy::needless_pass_by_value)]
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut l_idx: usize = 0;
        let mut r_idx: usize = 0;

        let mut high_score: i32 = 0;
        let mut score: i32 = 0;

        'outer: while r_idx < s.len() {
            let here = s.as_bytes()[r_idx];

            for i in l_idx..r_idx {
                if s.as_bytes()[i] == here {
                    // This window contains a duplicate character.
                    score -= 1;
                    l_idx += 1;

                    continue 'outer;
                }
            }

            score += 1;

            high_score = score.max(high_score);

            r_idx += 1;
        }

        high_score
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_0() {
        let s = "abcabcbb".to_string();
        let ans = 3;

        assert_eq!(Solution::length_of_longest_substring(s), ans);
    }

    #[test]
    fn case_1() {
        let s = "bbbbb".to_string();
        let ans = 1;

        assert_eq!(Solution::length_of_longest_substring(s), ans);
    }

    #[test]
    fn case_2() {
        let s = "pwwkew".to_string();
        let ans = 3;

        assert_eq!(Solution::length_of_longest_substring(s), ans);
    }

    #[test]
    fn case_3() {
        let s = "dvdf".to_string();
        let ans = 3;

        assert_eq!(Solution::length_of_longest_substring(s), ans);
    }
}
