struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        // I think we find the best profit first, then find next best profits
        // excluding the timeframe of the first. Repeat until there is no more possible profit.
        //
        // But then what if there's a profit of 50 to be gained, but there's also two disjoint
        // profits of 49 and 49 that overlap with the 50? This strat won't get us the highest
        // score. Is the question THAT advanced? Maybe
        todo!()
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
}
