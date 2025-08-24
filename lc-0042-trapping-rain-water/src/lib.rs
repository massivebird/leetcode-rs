struct Solution;

#[allow(unused, clippy::needless_pass_by_value)]
impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut ans = 0;

        let descending = false;

        let mut i = 0;

        while i < height.len() {
            let mut this = height[i];

            if height.get(i + 1).is_some_and(|next| *next >= this) {
                i += 1;
                continue;
            }

            while let Some(next) = height.get(i + 1)
                && *next < this
            {
                this = *next;
                i += 1;
            }

            // `i` now positioned at a local minimum.

            let (mut l, mut r) = (i, i);

            // Find trap's left bound.
            while let Some(prev) = height.get(l.saturating_sub(1))
                && *prev > height[l]
            {
                l -= 1;
            }

            // Find trap's right bound.
            while let Some(next) = height.get(r + 1)
                && *next > height[r]
            {
                r += 1;
            }

            let trap_height = i32::min(height[l], height[r]);

            let mut trap_volume = 0;

            for height in &height[l..r] {
                trap_volume += i32::max(0, trap_height - height);
            }

            println!("{i} {l} {r} {trap_volume}");

            ans += trap_volume;

            i += 1;
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_0() {
        let height = vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];
        let ans = 6;

        assert_eq!(Solution::trap(height), ans);
    }

    #[test]
    fn case_1() {
        let height = vec![4, 2, 0, 3, 2, 5];
        let ans = 9;

        assert_eq!(Solution::trap(height), ans);
    }
}
