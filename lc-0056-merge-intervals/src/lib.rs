struct Solution;

#[allow(unused, clippy::needless_pass_by_value)]
impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        todo!()
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
}
