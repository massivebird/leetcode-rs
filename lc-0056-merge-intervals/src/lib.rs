struct Solution;

#[allow(unused, clippy::needless_pass_by_value)]
impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut ans = vec![intervals[0].clone()];

        let overlap = |a: &[i32], b: &[i32]| {
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

        'outer: for this in intervals.into_iter().skip(1) {
            let mut merged = false;

            for other in &mut ans {
                if overlap(&this, other) {
                    merged = true;
                    other[0] = i32::min(this[0], other[0]);
                    other[1] = i32::max(this[1], other[1]);
                }
            }

            if !merged {
                ans.push(this.clone());
            }
        }

        let mut uniq_ans = Vec::new();

        for (idx, this) in ans.into_iter().enumerate() {
            if !uniq_ans.contains(&this) {
                uniq_ans.push(this);
            }
        }

        uniq_ans
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
