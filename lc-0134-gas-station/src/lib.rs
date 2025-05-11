struct Solution {}

#[allow(dead_code, clippy::needless_pass_by_value)]
impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_0() {
        let gas = vec![1, 2, 3, 4, 5];
        let cost = vec![3, 4, 5, 1, 2];
        assert_eq!(Solution::can_complete_circuit(gas, cost), 3);
    }

    #[test]
    fn case_1() {
        let gas = vec![2, 3, 4];
        let cost = vec![3, 4, 3];
        assert_eq!(Solution::can_complete_circuit(gas, cost), -1);
    }
}
