#[allow(unused)]
struct Solution;

impl Solution {
    #[allow(unused)]
    pub fn is_happy(mut n: i32) -> bool {
        let mut known_values = Vec::new();

        loop {
            n = n
                .to_string()
                .chars()
                .map(|c| c.to_digit(10).unwrap().pow(2))
                .sum::<u32>() as i32;

            println!("{n}");

            if n == 1 {
                return true;
            }

            if known_values.contains(&n) {
                println!("break {n}");
                break false;
            }

            known_values.push(n);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_0() {
        assert!(Solution::is_happy(19));
    }

    #[test]
    fn case_1() {
        assert!(!Solution::is_happy(2));
    }

    #[test]
    fn case_2() {
        assert!(Solution::is_happy(7));
    }
}
