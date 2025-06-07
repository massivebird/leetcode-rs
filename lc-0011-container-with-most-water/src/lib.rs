struct Solution {}

impl Solution {
    #[allow(unused, clippy::needless_pass_by_value)]
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut left_idx = 0;
        let mut right_idx = height.len() - 1;

        let mut record_area = 0;

        while left_idx < right_idx {
            let width: i32 = i32::try_from(right_idx - left_idx).unwrap();

            let left_height = height.get(left_idx).unwrap();
            let right_height = height.get(right_idx).unwrap();
            let bounded_height = left_height.min(right_height);

            let area = width * bounded_height;

            record_area = i32::max(record_area, area);

            if left_height < right_height {
                left_idx += 1;
            } else {
                right_idx -= 1;
            }
        }

        record_area
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(49, Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]));
        assert_eq!(1, Solution::max_area(vec![1, 1]));
    }
}
