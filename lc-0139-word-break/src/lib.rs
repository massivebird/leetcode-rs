//! Given a string s and a dictionary of strings wordDict, return true if s can be segmented into a space-separated sequence of one or more dictionary words.
//!
//! Note that the same word in the dictionary may be reused multiple times in the segmentation.
//!
//! MB: My dynamic programming approach! Based on "tracking" dictionary words.
//! New words are tracked _only if_ a word was previously completed. Goal is to
//! complete a word at the input string's final character.
//!
//!
//! E.g. `s = catba`    `word_dict = [ "car", "bar" ]`
//!             ^
//!           car, runs into a wrong character! Removed as incomplete.
//!           Doesn't track more words; answer is no.
//!
//!
//! E.g. `s = dogma`    `word_dict = [ "ma", "dog", "log" ]`
//!             ^ ^
//!             | |
//!           dog, fully matched. Tracks new words.
//!               |
//!               ma, fully matched at the last character; answer is yes.
//!
//!
//! Cool space optimization: instead of using a full DP table, I use a
//! single mutable cell, since any cell only iterates off the previous one.

mod tests;

struct Solution;

impl Solution {
    #[allow(dead_code, clippy::needless_pass_by_value)]
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        // Represents a cell in the DP table.
        // Pairs a word with a mutable index. The index represents the word's
        // "progress", i.e. an index of 2 states that the word's first 3
        // characters have matched in the input string so far.
        let mut buf: Vec<(&str, usize)> = Vec::with_capacity(word_dict.len());

        // Whether or not a full word was matched in the last loop iter.
        let mut prev_cmpd = false;

        for (i, c) in s.chars().enumerate() {
            // Progress all tracked words.
            for (_s, i) in &mut buf {
                *i += 1;
            }

            // Keep tracked words only if they continue to match.
            buf.retain(|(s, i)| s.chars().nth(*i).is_some_and(|o| o == c));

            // Remove completed words.
            let mut just_cmpd = false;
            buf.retain(|(s, i)| {
                let keep = *i < s.len() - 1;

                // Record if a word has just completed.
                just_cmpd |= !keep;

                keep
            });

            // Skip tracking new words if we hadn't completed any previously.
            if i > 0 && !prev_cmpd {
                prev_cmpd = just_cmpd;
                continue;
            }

            // Track new words.
            for word in &word_dict {
                // Ignore words that don't start with this char.
                if word.chars().nth(0).is_none_or(|o| o != c) {
                    continue;
                }

                // If this word is one char, mark as completed instead of
                // adding it.
                if word.len() == 1 {
                    just_cmpd = true;
                    continue;
                }

                buf.push((word, 0));
            }

            prev_cmpd = just_cmpd;
        }

        // If a word was completed on the final character, then the whole
        // input string was matched with sequential words!
        prev_cmpd
    }
}
