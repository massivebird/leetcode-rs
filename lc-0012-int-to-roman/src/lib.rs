struct Solution {}

#[allow(unused)]
impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        let mut n = num;

        let mut_contains = |n: &mut i32, d: i32| {
            if *n / d >= 1 {
                *n -= d;
                true
            } else {
                false
            }
        };

        let mut buf = String::new();

        while n >= 1 {
            if mut_contains(&mut n, 1000) {
                buf.push('M');
            } else if mut_contains(&mut n, 900) {
                buf.push_str("CM");
            } else if mut_contains(&mut n, 500) {
                buf.push('D');
            } else if mut_contains(&mut n, 400) {
                buf.push_str("CD");
            } else if mut_contains(&mut n, 100) {
                buf.push('C');
            } else if mut_contains(&mut n, 90) {
                buf.push_str("XC");
            } else if mut_contains(&mut n, 50) {
                buf.push('L');
            } else if mut_contains(&mut n, 40) {
                buf.push_str("XL");
            } else if mut_contains(&mut n, 10) {
                buf.push('X');
            } else if mut_contains(&mut n, 9) {
                buf.push_str("IX");
            } else if mut_contains(&mut n, 5) {
                buf.push('V');
            } else if mut_contains(&mut n, 4) {
                buf.push_str("IV");
            } else if mut_contains(&mut n, 1) {
                buf.push('I');
            }
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
