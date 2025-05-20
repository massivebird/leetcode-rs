struct Solution {}

#[allow(unused)]
impl Solution {
    pub fn int_to_roman(mut num: i32) -> String {
        let numerals = [
            ("M", 1000),
            ("CM", 900),
            ("D", 500),
            ("CD", 400),
            ("C", 100),
            ("XC", 90),
            ("L", 50),
            ("XL", 40),
            ("X", 10),
            ("IX", 9),
            ("V", 5),
            ("IV", 4),
            ("I", 1),
        ];

        let mut buf = String::new();

        while num >= 1 {
            let (symbol, val) = numerals.iter().find(|(_, val)| num / *val >= 1).unwrap();

            num -= *val;

            buf.push_str(symbol);
        }

        buf
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_0() {
        assert_eq!(Solution::int_to_roman(3749), "MMMDCCXLIX");
    }

    #[test]
    fn case_1() {
        assert_eq!(Solution::int_to_roman(58), "LVIII");
    }
}
