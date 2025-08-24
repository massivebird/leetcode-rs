struct Solution;

#[allow(unused, clippy::needless_pass_by_value)]
impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        todo!();
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
}
