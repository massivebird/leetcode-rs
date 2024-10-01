#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn can_arrange(arr: Vec<i32>, k: i32) -> bool {
        let mut map = vec![0; k as usize];

        for val in &arr {
            let idx = (val.abs() % k) as usize;
            *map.get_mut(idx).unwrap() += 1;
        }

        dbg!(&map);

        if map.first().unwrap() % 2 == 1 {
            return false;
        }

        for idx in 0..map.len() / 2 {
            if map.get(1 + idx).unwrap() != map.get(map.len() - 1 - idx).unwrap() {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert!(Solution::can_arrange(
            vec![1, 2, 3, 4, 5, 10, 6, 7, 8, 9],
            5
        ))
    }

    #[test]
    fn case_1() {
        assert!(Solution::can_arrange(vec![1, 2, 3, 4, 5, 6], 7))
    }

    #[test]
    fn case_2() {
        assert!(Solution::can_arrange(vec![-1,1,-2,2,-3,3,-4,4], 3))
    }
}
