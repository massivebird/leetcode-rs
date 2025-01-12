struct Solution;

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        if m > 0 && n == 0 {
            return;
        }

        if m == 0 && n > 0 {
            *nums1 = nums2.to_vec();
            return;
        }

        let mut nums1_idx: usize = 0;
        let mut nums2_idx: usize = 0;

        let mut result = Vec::new();

        while nums1_idx < m as usize || nums2_idx < n as usize {
            if nums1_idx < m as usize && nums1[nums1_idx] <= nums2[nums2_idx] {
                result.push(nums1[nums1_idx]);
                nums1_idx += 1;
            } else {
                result.push(nums2[nums2_idx]);
                nums2_idx += 1;
            }
        }

        *nums1 = result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let mut a = vec![1, 2, 3, 0, 0, 0];
        let mut b = vec![2, 5, 6];

        Solution::merge(&mut a, 3, &mut b, 3);

        assert_eq!(a, vec![1, 2, 2, 3, 5, 6]);
    }

    #[test]
    fn case_2() {
        let mut a = vec![0];
        let mut b = vec![1];

        Solution::merge(&mut a, 0, &mut b, 1);

        assert_eq!(a, vec![1]);
    }
}
