struct Solution;

impl Solution {
    #[allow(dead_code, clippy::needless_pass_by_value)]
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut all_product = 1;
        let mut all_product_nonzero = 1;
        let mut num_zeroes = 0;

        for &num in &nums {
            all_product *= num;

            if num == 0 {
                num_zeroes += 1;
            } else {
                all_product_nonzero *= num;
            }
        }

        let mut ans: Vec<i32> = vec![0; nums.len()];

        for (idx, &num) in nums.iter().enumerate() {
            if num == 0 {
                if num_zeroes == 1 {
                    ans[idx] = all_product_nonzero;
                }
                continue;
            }

            ans[idx] = all_product / num;
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_0() {
        assert_eq!(
            Solution::product_except_self(vec![1, 2, 3, 4]),
            vec![24, 12, 8, 6]
        );
    }

    #[test]
    fn case_1() {
        assert_eq!(
            Solution::product_except_self(vec![-1, 1, 0, -3, 3]),
            vec![0, 0, 9, 0, 0]
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::product_except_self(vec![0, 4, 0]), vec![0, 0, 0]);
    }
}
