struct Solution;

// Very similar to LC-0056: Merge Intervals

#[allow(unused, clippy::needless_pass_by_value)]
impl Solution {
    pub fn insert(mut intervals: Vec<Vec<i32>>, mut new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        // Sort by each interval's starting value.
        //
        // This positions overlapping intervals adjacent to each other.
        intervals.sort_by_key(|a| a[0]);

        let mut merged = false;

        let Some(mut idx) = intervals
            .iter()
            .position(|iv| iv[1] >= new_interval[0])
        else {
            intervals.push(new_interval);
            return intervals;
        };

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
}
