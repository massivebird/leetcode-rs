struct Solution;

#[allow(clippy::needless_pass_by_value, unused)]
impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut l: usize = 0;
        let mut r: usize = numbers.len() - 1;

        while l <= r {
            match (numbers[l] + numbers[r]).cmp(&target) {
                std::cmp::Ordering::Equal => {
                    return vec![i32::try_from(l + 1).unwrap(), i32::try_from(r + 1).unwrap()];
                }
                std::cmp::Ordering::Less => l += 1,
                std::cmp::Ordering::Greater => r -= 1,
            }
        }

        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_0() {
        let numbers = vec![2, 7, 11, 15];
        let target = 9;

        assert_eq!(Solution::two_sum(numbers, target), vec![1, 2]);
    }

    #[test]
    fn case_1() {
        let numbers = vec![2, 3, 4];
        let target = 6;
        let ans = vec![1, 3];

        assert_eq!(Solution::two_sum(numbers, target), ans);
    }

    #[test]
    fn case_2() {
        let numbers = vec![-1, 0];
        let target = -1;
        let ans = vec![1, 2];

        assert_eq!(Solution::two_sum(numbers, target), ans);
    }
}
