struct Solution {}

#[allow(dead_code, clippy::needless_pass_by_value)]
impl Solution {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        let total_citations = citations.len();

        let mut t = vec![0; total_citations + 1];

        let mut candidate_index = 0;
        let mut candidate_count = 0;

        for c in citations {
            for i in (1..=c as usize).take(usize::min(c as usize, total_citations)) {
                t[i] += 1;
                if t[i] >= i && i > candidate_index {
                    candidate_index = i;
                    candidate_count = t[i];
                }
            }
        }

        candidate_index as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_0() {
        assert_eq!(Solution::h_index(vec![3, 0, 6, 1, 5]), 3);
    }

    #[test]
    fn case_1() {
        assert_eq!(Solution::h_index(vec![1, 3, 1]), 1);
    }
}
