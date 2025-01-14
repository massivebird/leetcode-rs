use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut freqs: HashMap<i32, u32> = HashMap::new();

        freqs.insert(nums[0], 1);
        let mut majority_val: i32 = nums[0];
        let mut majority_freq: u32 = 1;

        for e in nums.into_iter().skip(1) {
            freqs.entry(e).and_modify(|f| *f += 1).or_insert(1);

            let this_freq = freqs.get(&e).unwrap();

            if e == majority_val {
                majority_freq += 1;
                continue;
            }

            if *this_freq > majority_freq {
                majority_val = e;
                majority_freq = *this_freq;
            }
        }

        majority_val
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(Solution::majority_element(vec![3,2,3]), 3);
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::majority_element(vec![6,5,5]), 5);
    }
}
