mod tests;

struct Solution;

impl Solution {
    #[allow(dead_code, clippy::needless_pass_by_value)]
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut best_ans = 1;

        let mut searched: Vec<usize> = Vec::new();
        let mut to_search: Vec<usize> = vec![0];

        while let Some(low_i) = to_search.pop() {
            searched.push(low_i);

            let this = nums[low_i];
            let mut to_beat = nums[low_i];
            let mut prev_to_beat = nums[low_i];
            let mut score = 1;

            for (i, &n) in nums.iter().enumerate().skip(low_i + 1) {
                // Potential new lowest.
                if n < this && !searched.contains(&i) {
                    to_search.push(i);
                }

                // Replace the latest to_beat.
                if n < to_beat && n > prev_to_beat && prev_to_beat != this {
                    to_beat = n;
                    prev_to_beat = n;
                    score += 1;
                    continue;
                }

                if n > to_beat {
                    dbg!((this, to_beat, n));
                    score += 1;

                    to_beat = n;

                    best_ans = i32::max(best_ans, score);
                }
            }
        }

        best_ans
    }
}
