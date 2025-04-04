struct Solution;

#[allow(dead_code)]
#[allow(clippy::needless_pass_by_value)]
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut grid: Vec<i32> = vec![0; prices.len() + 1];

        for (i, bought) in prices.iter().enumerate() {
            for (j, sold) in prices.iter().enumerate() {
                if j < i {
                    continue;
                }

                // Best profit up to buying on the current day.
                let base = grid[i];

                grid[j + 1] = i32::max(sold - bought + base, grid[j + 1]);
            }
        }

        grid[prices.len()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let prices = vec![7, 1, 5, 3, 6, 4];
        assert_eq!(Solution::max_profit(prices), 7);
    }

    #[test]
    fn case_2() {
        let prices = vec![1, 2, 3, 4, 5];
        assert_eq!(Solution::max_profit(prices), 4);
    }

    #[test]
    fn case_3() {
        let prices = vec![7, 6, 4, 3, 1];
        assert_eq!(Solution::max_profit(prices), 0);
    }

    #[test]
    fn case_4() {
        let prices = vec![6, 1, 3, 2, 4, 7];
        assert_eq!(Solution::max_profit(prices), 7);
    }
}
