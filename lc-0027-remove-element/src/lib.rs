struct Solution;

#[allow(unused)]
impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut new_nums: Vec<i32> = Vec::new();

        let mut num_not_matching = 0;

        for e in nums.iter() {
            if *e != val {
                new_nums.push(*e);
                num_not_matching += 1;
            }
        }

        *nums = new_nums;

        num_not_matching
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let mut a = vec![3, 2, 2, 3];
        assert_eq!(Solution::remove_element(&mut a, 3), 2);
        assert_eq!(a, vec![2, 2]);
    }
}
