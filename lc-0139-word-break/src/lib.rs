mod tests;

struct Solution;

impl Solution {
    #[allow(dead_code, clippy::needless_pass_by_value)]
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let mut buf: Vec<(&str, usize)> = Vec::with_capacity(s.len());
        let mut prev_cmpd = false;

        for (i, c) in s.chars().enumerate() {
            dbg!(c);
            // Remove tracked words that don't fit anymore.
            buf.retain(|(s, i)| s.chars().nth(i + 1).is_some_and(|o| o == c));

            dbg!(&buf);
            if i > 0 && !prev_cmpd && buf.is_empty() {
                return false;
            }

            for (_s, i) in &mut buf {
                *i += 1;
            }

            // Remove completed tracked words.
            let init_len = buf.len();
            buf.retain(|(s, i)| *i < s.len() - 1);
            let post_len = buf.len();

            let just_cmpd = init_len != post_len;

            for word in &word_dict {
                if
                // Skip words that end in the character that just completed
                // other words
                (!prev_cmpd && just_cmpd && word.chars().nth(0).is_some_and(|o| c == o))
                    // Skip words that don't start with this character
                    || word.chars().nth(0).is_none_or(|o| c != o)
                {
                    continue;
                }

                buf.push((word, 0));
            }

            let init_len = buf.len();
            buf.retain(|(s, i)| *i < s.len() - 1);
            let post_len = buf.len();

            let just_cmpd = just_cmpd || (init_len != post_len);

            if just_cmpd {
                prev_cmpd = true;

                // Completed on the last character!
                if i == s.len() - 1 {
                    return true;
                }

                continue;
            }

            prev_cmpd = false;
        }

        buf.iter().any(|(s, i)| *i == s.len() - 1)
    }
}
