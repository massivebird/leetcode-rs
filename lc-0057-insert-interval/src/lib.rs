struct Solution;

// Very similar to LC-0056: Merge Intervals

#[allow(unused, clippy::needless_pass_by_value)]
impl Solution {
    pub fn insert(mut intervals: Vec<Vec<i32>>, mut new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        // Sort by each interval's starting value.
        //
        // This positions overlapping intervals adjacent to each other.
        intervals.sort_by_key(|a| a[0]);

        let overlaps = |a: &[i32], b: &[i32]| {
            // `a` left-leaning
            // `a` starts after `b` starts AND starts before `b` ends
            (a[0] >= b[0] && a[0] <= b[1]) ||
            // `a` right-leaning
            // `a` ends before `b` ends AND ends after `b` starts
            (a[1] <= b[1] && a[1] >= b[0]) ||
            // `a` inside
            // `a` starts after `b` starts AND ends before `b` ends
            (a[0] >= b[0] && a[1] <= b[1]) ||
            // `a` outside
            // `a` starts before `b` starts AND ends after `b` ends
            (a[0] <= b[0] && a[1] >= b[1])
        };

        let mut merged = false;
        let mut merge_interval = (None, None);

        for (idx, iv) in intervals.clone().iter().enumerate() {
            match (overlaps(iv, &new_interval), merged) {
                // Start merging
                (true, false) => {
                    new_interval[0] = new_interval[0].min(iv[0]);
                    new_interval[1] = new_interval[1].max(iv[1]);

                    merge_interval.0 = Some(idx);
                    merged = true;
                }
                // Still merging
                (true, true) => {
                    new_interval[0] = new_interval[0].min(iv[0]);
                    new_interval[1] = new_interval[1].max(iv[1]);
                }
                // No longer merging
                (false, true) => {
                    merge_interval.1 = Some(idx);
                }
                // No merge yet
                _ => (),
            }
        }

        let (Some(merge_start), Some(merge_end)) = merge_interval else {
            let pos = intervals
                .iter()
                .position(|iv| iv[0] >= new_interval[0])
                .unwrap_or(intervals.len());

            intervals.insert(pos, new_interval);

            return intervals;
        };

        for _ in merge_start..merge_end {
            intervals.remove(merge_start);
        }

        intervals.insert(merge_start, new_interval);

        intervals
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_0() {
        let intervals = [[1, 2], [3, 5], [6, 7], [8, 10], [12, 16]]
            .iter()
            .map(Vec::from)
            .collect::<Vec<Vec<i32>>>();

        let new_interval = vec![4, 8];

        let ans = [[1, 2], [3, 10], [12, 16]]
            .iter()
            .map(Vec::from)
            .collect::<Vec<Vec<i32>>>();

        assert_eq!(Solution::insert(intervals, new_interval), ans);
    }

    #[test]
    fn case_1() {
        let intervals = [[1, 3], [6, 9]]
            .iter()
            .map(Vec::from)
            .collect::<Vec<Vec<i32>>>();

        let new_interval = vec![2, 5];

        let ans = [[1, 5], [6, 9]]
            .iter()
            .map(Vec::from)
            .collect::<Vec<Vec<i32>>>();

        assert_eq!(Solution::insert(intervals, new_interval), ans);
    }

    #[test]
    fn case_2() {
        let intervals = [[1, 3], [6, 9]]
            .iter()
            .map(Vec::from)
            .collect::<Vec<Vec<i32>>>();

        let new_interval = vec![10, 12];

        let ans = [[1, 3], [6, 9], [10, 12]]
            .iter()
            .map(Vec::from)
            .collect::<Vec<Vec<i32>>>();

        assert_eq!(Solution::insert(intervals, new_interval), ans);
    }

    #[test]
    fn case_3() {
        let intervals = vec![vec![1, 5]];

        let new_interval = vec![2, 3];

        let ans = vec![vec![1, 5]];

        assert_eq!(Solution::insert(intervals, new_interval), ans);
    }
}
