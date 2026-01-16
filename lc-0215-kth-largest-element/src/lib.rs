struct Solution;

#[allow(dead_code)]
impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        use std::collections::BinaryHeap;

        let mut heap: BinaryHeap<i32> = BinaryHeap::new();

        for num in nums {
            heap.push(num);
        }

        heap.into_sorted_vec()
            .into_iter()
            .rev()
            .nth(usize::try_from(k - 1).unwrap())
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_0() {
        let nums = vec![3, 2, 1, 5, 6, 4];
        let k = 2;
        let ans = 5;

        assert_eq!(Solution::find_kth_largest(nums, k), ans);
    }

    #[test]
    fn case_1() {
        let nums = vec![3, 2, 3, 1, 2, 4, 5, 5, 6];
        let k = 4;
        let ans = 4;

        assert_eq!(Solution::find_kth_largest(nums, k), ans);
    }
}
