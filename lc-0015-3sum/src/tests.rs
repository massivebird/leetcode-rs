#![cfg(test)]

use crate::Solution;

#[test]
fn case_0() {
    let nums = [-1, 0, 1, 2, -1, -4];
    let ans = [[-1, -1, 2], [-1, 0, 1]];

    assert_eq!(Solution::three_sum(nums.into()), ans);
}

#[test]
fn case_1() {
    let nums = [-100, -70, -60, 110, 120, 130, 160];
    let ans = [[-100, -60, 160], [-70, -60, 130]];

    assert_eq!(Solution::three_sum(nums.into()), ans);
}
