struct Solution {}

#[allow(dead_code, clippy::needless_pass_by_value)]
impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let costs = cost;

        let mut tank: i32 = 0;

        'outer: for (idx, &cost) in costs.iter().enumerate() {
            if cost > gas[idx] {
                tank = 0;
                continue;
            }

            tank += gas[idx];
            tank -= costs[idx];

            for (jdx, &cost) in costs
                .iter()
                .enumerate()
                .cycle()
                .skip(idx + 1)
                .take(costs.len())
            {
                tank += gas[jdx];

                if cost > tank {
                    tank = 0;
                    continue 'outer;
                }

                tank -= costs[jdx];
            }

            return idx as i32;
        }

        -1
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
