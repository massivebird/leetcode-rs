struct Solution {}

#[allow(dead_code, clippy::needless_pass_by_value)]
impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let costs = cost;

        // Track total cost in the arena.
        let mut global_cost = 0;
        // Track total amount of gas in the arena.
        let mut global_gas = 0;

        let mut tank: i32 = 0;
        let mut start_idx = 0;

        for idx in 0..costs.len() {
            global_cost += costs[idx];
            global_gas += gas[idx];

            tank = tank + gas[idx] - costs[idx];

            if tank < 0 {
                // Current start index could not support the trip.
                start_idx = idx + 1;
                tank = 0;
            }
        }

        if global_gas < global_cost || start_idx > costs.len() || tank < 0 {
            return -1;
        }

        start_idx as i32
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

    #[test]
    fn case_2() {
        let gas = vec![0, 0, 0, 2];
        let cost = vec![0, 0, 1, 0];
        assert_eq!(Solution::can_complete_circuit(gas, cost), 3);
    }
}
