struct Solution;

#[allow(unused, clippy::needless_pass_by_value)]
impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 {
            return s;
        }

        let mut rows = vec![String::new(); usize::try_from(num_rows).unwrap()];

        let mut row_idx = 0;
        let mut ascending = false;

        for c in s.chars() {
            rows[row_idx].push(c);

            row_idx = match (row_idx, ascending) {
                (0, _) => {
                    ascending = false;
                    1
                }
                (x, _) if i32::try_from(x).unwrap() == num_rows - 1 => {
                    ascending = true;
                    x - 1
                }
                (x, true) => x - 1,
                (x, false) => x + 1,
            };
        }

        rows.into_iter().collect::<String>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_0() {
        assert_eq!(
            Solution::convert("PAYPALISHIRING".to_string(), 3),
            "PAHNAPLSIIGYIR".to_string()
        );
    }

    #[test]
    fn case_1() {
        assert_eq!(
            Solution::convert("PAYPALISHIRING".to_string(), 4),
            "PINALSIGYAHRPI".to_string()
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::convert("A".to_string(), 1), "A".to_string());
    }

    #[test]
    fn case_3() {
        assert_eq!(
            Solution::convert("AB".to_string(), 1),
            "AB".to_string()
        );
    }
}
