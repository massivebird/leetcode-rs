struct Solution;

// Input is a list of two-value ranges, e.g. `[[1,3], [2,4]]`.
//
// Return a list of these two-element ranges, merged whenever possible such
// that none in the returned list overlap.

impl Solution {
    #[allow(dead_code, clippy::needless_pass_by_value)]
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        // Sort by each interval's starting value.
        //
        // This positions overlapping intervals adjacent to each other.
        intervals.sort_by_key(|a| a[0]);

        let mut ans: Vec<Vec<i32>> = Vec::new();

        let mut idx: usize = 0;

        while idx < intervals.len() {
            let mut this = intervals[idx].clone();

            idx += 1;

            while let Some(other) = intervals.get(idx) {
                // Exhausted all overlapping intervals.
                if other[0] > this[1] {
                    break;
                }

                // Absorb the overlapping interval.
                //
                // Since the intervals are sorted by start idx, the start
                // idx is already at its minimum.
                this[1] = i32::max(this[1], other[1]);

                idx += 1;
            }

            ans.push(this);
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_0() {
        let intervals = [[1, 3], [2, 6], [8, 10], [15, 18]]
            .iter()
            .map(Vec::from)
            .collect::<Vec<Vec<i32>>>();

        let ans = [[1, 6], [8, 10], [15, 18]]
            .iter()
            .map(Vec::from)
            .collect::<Vec<Vec<i32>>>();

        assert_eq!(Solution::merge(intervals), ans);
    }

    #[test]
    fn case_1() {
        let intervals = [[1, 4], [4, 5]]
            .iter()
            .map(Vec::from)
            .collect::<Vec<Vec<i32>>>();

        let ans = vec![vec![1, 5]];

        assert_eq!(Solution::merge(intervals), ans);
    }

    #[test]
    fn case_2() {
        let intervals = [[1, 4], [0, 5]]
            .iter()
            .map(Vec::from)
            .collect::<Vec<Vec<i32>>>();

        let ans = vec![vec![0, 5]];

        assert_eq!(Solution::merge(intervals), ans);
    }

    #[test]
    fn case_3() {
        let intervals = [[2, 3], [4, 5], [6, 7], [8, 9], [1, 10]]
            .iter()
            .map(Vec::from)
            .collect::<Vec<Vec<i32>>>();

        let ans = vec![vec![1, 10]];

        assert_eq!(Solution::merge(intervals), ans);
    }

    #[test]
    fn case_4() {
        let intervals = [[2, 3], [4, 6], [5, 7], [3, 4]]
            .iter()
            .map(Vec::from)
            .collect::<Vec<Vec<i32>>>();

        let ans = vec![vec![2, 7]];

        assert_eq!(Solution::merge(intervals), ans);
    }
}
