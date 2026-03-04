mod tests;

struct Solution;

impl Solution {
    #[allow(dead_code, clippy::needless_pass_by_value)]
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort_unstable();

        let mut ans: Vec<Vec<i32>> = Vec::new();

        for m in 0..nums.len() {
            let mut l: usize = m + 1;
            let mut r: usize = nums.len() - 1;

            while l < r {
                let sum = nums[l] + nums[m] + nums[r];

                if sum > 0 {
                    r -= 1;
                    continue;
                } else if sum < 0 {
                    l += 1;
                    continue;
                }

                let mut vec = vec![nums[l], nums[m], nums[r]];
                vec.sort_unstable();

                if !ans.contains(&vec) {
                    ans.push(vec);
                }

                l += 1;
            }
        }

        ans.into_iter().collect()
    }
}
