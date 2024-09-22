pub fn find_the_longest_substring(s: String) -> i32 {
    let mut end_idx: usize = 0;
    // we'll late-initialize the start_idx
    let mut start_idx: Option<usize> = None;

    let mut vowels = [0, 0, 0, 0, 0];

    for (idx, char) in s.char_indices() {
        match char {
            'a' => vowels[0] = (vowels[0] + 1) % 2,
            'e' => vowels[1] = (vowels[1] + 1) % 2,
            'i' => vowels[2] = (vowels[2] + 1) % 2,
            'o' => vowels[3] = (vowels[3] + 1) % 2,
            'u' => vowels[4] = (vowels[4] + 1) % 2,
            _ => (),
        }

        dbg!((idx, vowels));
        if vowels.iter().all(|c| *c == 0) {
            if start_idx.is_none() {
                start_idx = Some(idx);
            }
            end_idx = idx;
        }
    }

    dbg!((start_idx, end_idx));
    (end_idx - start_idx.unwrap_or(0) + 1) as i32
}

pub fn _find_the_longest_substring_old(s: String) -> i32 {
    let is_vowelly_valid = |s: &str| {
        for vowel in ['a', 'e', 'i', 'o', 'u'] {
            if s.chars().filter(|c| *c == vowel).count() % 2 != 0 {
                return false;
            }
        }
        true
    };

    let mut high_score: usize = 0;

    // Startup cancel if s itself is the answer
    if is_vowelly_valid(&s) {
        return s.len() as i32;
    }

    for idx in 0..s.len() {
        for jdx in idx..s.len() {
            let substr = &s[idx..=jdx];
            let len = jdx - idx + 1;

            if len > high_score && is_vowelly_valid(substr) {
                high_score = len;
            }
        }
    }

    high_score as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            find_the_longest_substring(String::from("leetcodeisgreat")),
            5
        );
        assert_eq!(find_the_longest_substring(String::from("bcbcbc")), 6);
        assert_eq!(find_the_longest_substring(String::from("id")), 1);
        assert_eq!(
            find_the_longest_substring(String::from("eleetminicoworoep")),
            13
        );
    }
}
