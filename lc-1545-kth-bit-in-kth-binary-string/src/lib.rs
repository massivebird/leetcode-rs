struct Solution;

impl Solution {
    #[allow(dead_code, clippy::needless_pass_by_value)]
    pub fn find_kth_bit(n: i32, k: i32) -> char {
        Self::generate_nth_bit_string(n)
            .chars()
            .nth(usize::try_from(k).unwrap() - 1)
            .unwrap()
    }

    fn generate_nth_bit_string(n: i32) -> String {
        let mut working_str = "0".to_string();

        let reverse_bit = |c: char| match c {
            '1' => '0',
            '0' => '1',
            _ => unreachable!(),
        };

        for _ in 0..n {
            working_str.push('1');
            working_str.push_str(
                &working_str
                    .clone()
                    .chars()
                    .rev()
                    .skip(1)
                    .map(&reverse_bit)
                    .collect::<String>(),
            );
        }

        working_str
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spec_examples() {
        assert_eq!(Solution::find_kth_bit(3, 1), '0');
        assert_eq!(Solution::find_kth_bit(4, 11), '1');
    }
}
