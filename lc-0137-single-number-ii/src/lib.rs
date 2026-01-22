struct Solution;

impl Solution {
    #[allow(dead_code, clippy::needless_pass_by_value)]
    pub fn single_number(nums: Vec<i32>) -> i32 {
        // NOTE: totally NOT a good answer.
        // The ""REAL"" answer uses a clever combination of bitwise ops.

        use std::collections::HashMap;

        let mut freqs: HashMap<i32, u32> = HashMap::new();

        for n in nums {
            freqs.entry(n).and_modify(|v| *v += 1).or_insert(1);
        }

        *freqs.iter().find(|(_, v)| **v == 1).unwrap().0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_0() {
        let nums = vec![2, 2, 3, 2];
        let ans = 3;

        assert_eq!(Solution::single_number(nums), ans);
    }

    #[test]
    fn case_1() {
        let nums = vec![0, 1, 0, 1, 0, 1, 99];
        let ans = 99;

        assert_eq!(Solution::single_number(nums), ans);
    }
}
