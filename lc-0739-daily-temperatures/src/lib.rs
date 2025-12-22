struct Solution;

#[allow(dead_code)]
impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut buf: Vec<i32> = vec![0; temperatures.len()];
        let mut stack: Vec<usize> = vec![];

        for (i, this) in temperatures.iter().enumerate() {
            // Pop all that are lesser
            while let Some(j) = stack.pop_if(|j| temperatures[*j] < *this) {
                buf[j] = i32::try_from(i - j).unwrap();
            }

            // Push this to stack
            stack.push(i);
        }

        buf
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_0() {
        let temps = vec![73, 74, 75, 71, 69, 72, 76, 73];
        let answer = vec![1, 1, 4, 2, 1, 1, 0, 0];
        assert_eq!(Solution::daily_temperatures(temps), answer);
    }
}
