// Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.
//
// You may assume that each input would have exactly one solution, and you may not use the same element twice.
//
// You can return the answer in any order.

fn main() {}

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for idx in 0..nums.len() - 1 {
        let diff = target - nums[idx];
        for (jdx, other) in nums.iter().enumerate().skip(idx + 1) {
            if *other == diff {
                return vec![idx as i32, jdx as i32];
            }
        }
    }
    unreachable!();
}

#[cfg(test)]
mod tests {
    use super::two_sum;

    #[test]
    fn whatever() {
        assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![0, 1])
    }
}
