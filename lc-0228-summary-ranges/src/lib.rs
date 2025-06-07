struct Solution;

#[allow(unused, clippy::needless_pass_by_value)]
impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        if nums.is_empty() {
            return Vec::new();
        }

        let mut ans = Vec::new();

        let mut l_val: i32 = nums[0];
        let mut r_val: i32 = nums[0];

        let mut push_range = |l_val: i32, r_val: i32| {
            ans.push(if l_val == r_val {
                l_val.to_string()
            } else {
                format!("{l_val}->{r_val}")
            });
        };

        for num in nums.iter().skip(1) {
            // Push and reset current range if this num breaks the streak.
            if *num != r_val + 1 {
                push_range(l_val, r_val);

                l_val = *num;
            }

            r_val = *num;
        }

        push_range(l_val, r_val);

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_0() {
        let nums = vec![0, 1, 2, 4, 5, 7];
        let ans: Vec<String> = ["0->2", "4->5", "7"]
            .iter()
            .map(|&str| str.to_owned())
            .collect();

        assert_eq!(Solution::summary_ranges(nums), ans);
    }

    #[test]
    fn case_1() {
        let nums = vec![0, 2, 3, 4, 6, 8, 9];
        let ans: Vec<String> = ["0", "2->4", "6", "8->9"]
            .iter()
            .map(|&str| str.to_owned())
            .collect();

        assert_eq!(Solution::summary_ranges(nums), ans);
    }
}
