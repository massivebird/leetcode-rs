#![allow(dead_code)]
struct Solution;

impl Solution {
    // Returns the maximum profit to be gained given an array of stock price
    // over time.
    //
    // Stock must be "bought" before it can be "sold".
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut best_profit = 0;
        let mut min = *prices.first().unwrap();

        for val in prices.into_iter().skip(1) {
            if val < min {
                min = val;
            }

            if val > min && val - min > best_profit {
                best_profit = val - min;
            }
        }

        best_profit
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
    }
}
