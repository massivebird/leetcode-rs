#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn can_arrange(arr: Vec<i32>, k: i32) -> bool {
        // Contains the frequencies of all possible remainder values `val % k`.
        let mut rem_freqs = vec![0; k as usize];

        for val in &arr {
            // `(val % k + k) % k` is an interesting expression. The goal is to
            // produce a positive remainder whether the input is positive or
            // negative.
            //
            // For val >= 0, the above is equivalent to `val % k`.
            //
            // For val < 0, the above:
            //   1. Computes `val % k` => (-k + 1)..=0,
            //   2. Adds `k` => 1..=k,
            //   3. Mod `k` => 0..k.
            let idx = ((val % k + k) % k) as usize;
            *rem_freqs.get_mut(idx).unwrap() += 1;
        }

        if rem_freqs.first().unwrap() % 2 == 1 {
            return false;
        }

        for idx in 0..rem_freqs.len() / 2 {
            if rem_freqs.get(1 + idx).unwrap() != rem_freqs.get(rem_freqs.len() - 1 - idx).unwrap()
            {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert!(Solution::can_arrange(
            vec![1, 2, 3, 4, 5, 10, 6, 7, 8, 9],
            5
        ))
    }

    #[test]
    fn case_1() {
        assert!(Solution::can_arrange(vec![1, 2, 3, 4, 5, 6], 7))
    }

    #[test]
    fn case_2() {
        assert!(Solution::can_arrange(vec![-1, 1, -2, 2, -3, 3, -4, 4], 3))
    }
}
