struct Solution;

#[allow(dead_code)]
impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        use std::cmp::Ordering;

        let mut l: usize = 0;
        let mut r: usize = nums.len() - 1;

        // Short-circuit if this element does not fit within the bounds
        // of this array.
        if target > nums[r] {
            return i32::try_from(nums.len()).unwrap();
        } else if target < nums[l] {
            return 0;
        }

        let mut i;

        loop {
            i = l.midpoint(r);
            // dbg!((l, r, i));

            let test = nums[i];

            match test.cmp(&target) {
                Ordering::Equal => break,

                // Search has completed without finding the target.
                // Return the index where it would be inserted.
                Ordering::Less if l == i => {
                    i += 1;
                    break;
                }
                Ordering::Greater if r == i => break,

                // More searching required!
                Ordering::Less => l = i,
                Ordering::Greater => r = i,
            }
        }

        i32::try_from(i).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_0() {
        let nums = vec![1, 3, 5, 6];
        let target = 5;
        let ans = 2;

        assert_eq!(Solution::search_insert(nums, target), ans);
    }

    #[test]
    fn case_1() {
        let nums = vec![1, 3, 5, 6];
        let target = 2;
        let ans = 1;

        assert_eq!(Solution::search_insert(nums, target), ans);
    }

    #[test]
    fn case_2() {
        let nums = vec![1, 3, 5, 6];
        let target = 7;
        let ans = 4;

        assert_eq!(Solution::search_insert(nums, target), ans);
    }

    #[test]
    fn case_3() {
        let nums = vec![1, 3];
        let target = 2;
        let ans = 1;

        assert_eq!(Solution::search_insert(nums, target), ans);
    }

    #[test]
    fn case_4() {
        let nums = vec![1, 3, 5];
        let target = 1;
        let ans = 0;

        assert_eq!(Solution::search_insert(nums, target), ans);
    }
}
