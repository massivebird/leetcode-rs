//! Given an array of strings words and a width maxWidth,
//! format the text such that each line has exactly maxWidth
//! characters and is fully (left and right) justified EXCEPT
//! that the last line is left-aligned.

struct Solution;

#[allow(unused, clippy::needless_pass_by_value)]
impl Solution {
    pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
        // Will contain the words present in each line.
        let mut lines: Vec<Vec<String>> = vec![vec![]];

        // Calculates the cumulative length of all strings in a vector.
        let acc_len = |v: &[String]| v.iter().fold(0, |acc, s| acc + s.len());

        // Place all words onto their output lines.
        for word in words {
            let line_width = acc_len(lines.last().unwrap());

            let num_words = lines.last().unwrap().len();
            let num_spaces = usize::try_from(max_width).unwrap() - line_width;

            // Test if there are enough spaces to accomodate this word, given
            // that `num_spaces` must not exceed `num_words - 1`.
            if num_spaces.saturating_sub(word.len()) >= num_words {
                lines.last_mut().unwrap().push(word);
            } else {
                // Not enough spaces remaining on this line.
                // Start a new line.
                lines.push(vec![word]);
            }
        }

        // Will contain the returned solution.
        let mut ans_vec: Vec<String> = Vec::new();

        // Justify-align each line.
        // Except for the last line, which is left-aligned.
        for line in lines.iter().take(lines.len() - 1) {
            // Will contain this line, justified.
            let mut line_buf = String::new();

            let acc_word_len = acc_len(line);

            let total_spaces = usize::try_from(max_width).unwrap() - acc_word_len;

            let num_words = line.len();

            if num_words <= 1 {
                ans_vec.push(format!("{}{}", line[0], " ".repeat(total_spaces)));
                continue;
            }

            let spaces_per_word = total_spaces / (num_words - 1);

            // Allocate extra spaces from left to right.
            let added_spaces_idx = total_spaces % (num_words - 1);

            for (idx, word) in line.iter().enumerate() {
                line_buf.push_str(word);
                line_buf.push_str(&" ".repeat(spaces_per_word));

                if added_spaces_idx > 0 && idx < added_spaces_idx {
                    line_buf.push(' ');
                }
            }

            ans_vec.push(line_buf.trim_end().to_owned());
        }

        // Left-justify the last line.

        let last_line = {
            let binding = lines
                .into_iter()
                .next_back()
                .unwrap()
                .into_iter()
                .reduce(|acc, s| acc + " " + &s)
                .unwrap();

            binding.trim_end().to_owned()
        };

        ans_vec.push(format!(
            "{last_line}{}",
            " ".repeat(usize::try_from(max_width).unwrap() - last_line.len())
        ));

        ans_vec
    }
}

#[cfg(test)]
mod tests {
    use std::string::ToString;

    use super::*;

    #[test]
    fn case_0() {
        let words = vec![
            "This".to_string(),
            "is".to_string(),
            "an".to_string(),
            "example".to_string(),
            "of".to_string(),
            "text".to_string(),
            "justification.".to_string(),
        ];

        let output = vec![
            "This    is    an".to_string(),
            "example  of text".to_string(),
            "justification.  ".to_string(),
        ];

        assert_eq!(Solution::full_justify(words, 16), output);
    }

    #[test]
    fn case_1() {
        let words = ["What", "must", "be", "acknowledgment", "shall", "be"];
        let words: Vec<String> = words.iter().map(ToString::to_string).collect();

        let output = ["What   must   be", "acknowledgment  ", "shall be        "];
        let output: Vec<String> = output.iter().map(ToString::to_string).collect();

        assert_eq!(Solution::full_justify(words, 16), output);
    }

    #[test]
    fn case_2() {
        let words = [
            "Science",
            "is",
            "what",
            "we",
            "understand",
            "well",
            "enough",
            "to",
            "explain",
            "to",
            "a",
            "computer.",
            "Art",
            "is",
            "everything",
            "else",
            "we",
            "do",
        ];
        let words: Vec<String> = words.iter().map(ToString::to_string).collect();

        let output = [
            "Science  is  what we",
            "understand      well",
            "enough to explain to",
            "a  computer.  Art is",
            "everything  else  we",
            "do                  ",
        ];
        let output: Vec<String> = output.iter().map(ToString::to_string).collect();

        assert_eq!(Solution::full_justify(words, 20), output);
    }

    #[test]
    fn case_3() {
        let words = [
            "The",
            "important",
            "thing",
            "is",
            "not",
            "to",
            "stop",
            "questioning.",
            "Curiosity",
            "has",
            "its",
            "own",
            "reason",
            "for",
            "existing.",
        ];
        let words: Vec<String> = words.iter().map(ToString::to_string).collect();

        let output = [
            "The     important",
            "thing  is  not to",
            "stop questioning.",
            "Curiosity has its",
            "own   reason  for",
            "existing.        ",
        ];
        let output: Vec<String> = output.iter().map(ToString::to_string).collect();

        assert_eq!(Solution::full_justify(words, 17), output);
    }

    #[test]
    fn case_4() {
        let words = ["Listen", "to", "many,", "speak", "to", "a", "few."];
        let words: Vec<String> = words.iter().map(ToString::to_string).collect();

        let output = ["Listen", "to    ", "many, ", "speak ", "to   a", "few.  "];
        let output: Vec<String> = output.iter().map(ToString::to_string).collect();

        assert_eq!(Solution::full_justify(words, 6), output);
    }
}
