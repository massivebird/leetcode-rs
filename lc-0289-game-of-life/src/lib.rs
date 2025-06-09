struct Solution;

#[allow(unused, clippy::needless_pass_by_ref_mut, clippy::ptr_arg)]
impl Solution {
    pub fn game_of_life(board: &mut Vec<Vec<i32>>) {
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_0() {
        let mut board = vec![vec![0, 1, 0], vec![0, 0, 1], vec![1, 1, 1], vec![0, 0, 0]];

        let ans = vec![vec![0, 0, 0], vec![1, 0, 1], vec![0, 1, 1], vec![0, 1, 0]];

        Solution::game_of_life(&mut board);

        assert_eq!(board, ans);
    }
}
