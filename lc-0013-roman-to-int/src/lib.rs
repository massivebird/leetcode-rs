struct Solution {}

#[allow(unused, clippy::needless_pass_by_value)]
impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let to_val = |c: char| -> i32 {
            match c {
                'I' => 1,
                'V' => 5,
                'X' => 10,
                'L' => 50,
                'C' => 100,
                'D' => 500,
                'M' => 1000,
                _ => unreachable!(),
            }
        };

        let mut sum = 0;

        let mut i = 0;
        while i < s.len() - 1 {
            let this = s.chars().nth(i).unwrap();
            let next = s.chars().nth(i + 1).unwrap();

            if to_val(this) < to_val(next) {
                sum += to_val(next) - to_val(this);
                i += 1;
            } else {
                sum += to_val(this);
            }

            i += 1;
        }

        // Stopped before last character.
        // Add it to the sum.
        if i == s.len() - 1 {
            sum += to_val(s.chars().last().unwrap());
        }

        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_0() {
        assert_eq!(Solution::roman_to_int("III".to_string()), 3);
    }

    #[test]
    fn case_1() {
        assert_eq!(Solution::roman_to_int("LVIII".to_string()), 58);
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::roman_to_int("MCMXCIV".to_string()), 1994);
    }
}
