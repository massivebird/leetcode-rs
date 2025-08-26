struct Solution;

#[allow(unused, clippy::needless_pass_by_value)]
impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut ans = 0;

        let descending = false;

        let mut m_idx = 0;

        while m_idx < height.len() {
            let mut this = height[m_idx];

            if height.get(m_idx + 1).is_some_and(|next| *next >= this) {
                m_idx += 1;
                continue;
            }

            while let Some(next) = height.get(m_idx + 1)
                && *next < this
            {
                this = *next;
                m_idx += 1;
            }

            // `i` now positioned at a local minimum.

            // Find trap's left bound.
            let mut l_height = 0;
            let mut l_idx = m_idx.saturating_sub(1);
            for (idx, l_opt) in height.iter().enumerate().take(m_idx).rev() {
                if *l_opt > l_height {
                    l_height = l_height.max(height[l_idx]);
                    l_idx = idx;
                }
            }

            // Find trap's right bound.
            let mut r_idx = usize::max(m_idx, height.len() - 1);
            let mut r_height = height[r_idx];
            for (idx, r_opt) in height.iter().enumerate().skip(m_idx) {
                if *r_opt > r_height {
                    r_height = *r_opt;
                    r_idx = idx;
                }

                if *r_opt > l_height {
                    break;
                }
            }

            let l_height = height[l_idx];
            let r_height = height[r_idx];
            let trap_height = i32::min(l_height, r_height);

            let mut trap_volume = 0;

            for height in &height[l_idx..r_idx] {
                // let height = i32::min(trap_height, *height);

                trap_volume += i32::max(0, trap_height - height);
            }

            println!("@{m_idx} @{l_idx} @{r_idx} V{trap_volume}");

            ans += trap_volume;

            m_idx = r_idx + 1;
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

    #[test]
    fn case_2() {
        let height = vec![5, 5, 1, 7, 1, 1, 5, 2, 7, 6];
        let ans = 23;

        assert_eq!(Solution::trap(height), ans);
    }

    #[test]
    fn case_3() {
        let height = vec![
            6, 4, 2, 0, 3, 2, 0, 3, 1, 4, 5, 3, 2, 7, 5, 3, 0, 1, 2, 1, 3, 4, 6, 8, 1, 3,
        ];
        let ans = 83;

        assert_eq!(Solution::trap(height), ans);
    }
}
