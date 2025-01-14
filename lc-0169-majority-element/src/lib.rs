use std::collections::HashMap;

struct Solution;

impl Solution {
    // KEY: assume some majority element always exists.
    // We can quit as soon as a value's freq exceeds floor(n/2).
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut freqs: HashMap<i32, u32> = HashMap::new();

        let goal = u32::try_from(nums.len() / 2).unwrap();

        for n in nums {
            freqs.entry(n).and_modify(|f| *f += 1).or_insert(1);

            if *freqs.get(&n).unwrap() > goal {
                return n;
            }
        }

        unreachable!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(Solution::majority_element(vec![3, 2, 3]), 3);
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::majority_element(vec![6, 5, 5]), 5);
    }
}
