// Courtesy of leetcode user hemanth00405.
//
// Overarching idea: we can use a 5-bit mask to track if we've encountered each
// vowel an even (0) or odd (1) number of times. The longest substring that
// contains an even number of vowels is initialized and terminated with
// the same bit mask!
//
// We'll leverage this to record when we first encounter each possible bitmask
// (if at all).
pub fn find_the_longest_substring(s: String) -> i32 {
    #[derive(Copy, Clone)]
    enum MaskStatus {
        NotFound,
        InitialIdx(i32), // i32 so we can set -1 for mask 00000 (whole string).
    }

    // Represents all possible 32 permutations of a 5-bit string.
    let mut mask_records = [MaskStatus::NotFound; 32];
    // Account for the possibility of the entire string being the answer.
    mask_records[0] = MaskStatus::InitialIdx(-1);

    // We will continuously mutate this mask as we go.
    let mut working_mask = 0;
    // The final answer, continuously updated as we go.
    let mut working_max_len = 0;

    for (idx, c) in s.char_indices() {
        let idx = idx as i32;

        match c {
            'a' => working_mask ^= 1,
            'e' => working_mask ^= 2,
            'i' => working_mask ^= 4,
            'o' => working_mask ^= 8,
            'u' => working_mask ^= 16,
            _ => (),
        }

        match mask_records[working_mask] {
            MaskStatus::NotFound => mask_records[working_mask] = MaskStatus::InitialIdx(idx),
            MaskStatus::InitialIdx(init_idx) => {
                working_max_len = i32::max(working_max_len, idx - init_idx)
            }
        }
    }

    working_max_len
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_leetcodeisgreat() {
        assert_eq!(
            find_the_longest_substring(String::from("leetcodeisgreat")),
            5
        );
    }

    #[test]
    fn case_bcbcbc() {
        assert_eq!(find_the_longest_substring(String::from("bcbcbc")), 6);
    }

    #[test]
    fn case_id() {
        assert_eq!(find_the_longest_substring(String::from("id")), 1);
    }

    #[test]
    fn case_eleetminicoworoep() {
        assert_eq!(
            find_the_longest_substring(String::from("eleetminicoworoep")),
            13
        );
    }
}
