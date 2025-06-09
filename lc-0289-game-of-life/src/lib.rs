struct Solution;

#[allow(unused, clippy::needless_pass_by_ref_mut, clippy::ptr_arg)]
impl Solution {
    pub fn game_of_life(board: &mut Vec<Vec<i32>>) {
        let width: usize = board[0].len();

        let mut ans = vec![vec![0; width]; board.len()];

        for row in 0..board.len() {
            for col in 0..width {
                let mut live_neighbors = 0u32;

                for tr in [
                    (-1, -1),
                    (-1, 0),
                    (-1, 1),
                    (0, -1),
                    (0, 1),
                    (1, -1),
                    (1, 0),
                    (1, 1),
                ] {
                    let Some(row) = (row as i32)
                        .checked_add(tr.0)
                        .filter(|v| *v >= 0 && *v < board.len() as i32)
                    else {
                        continue;
                    };

                    let Some(col) = (col as i32)
                        .checked_add(tr.1)
                        .filter(|v| *v >= 0 && *v < width as i32)
                    else {
                        continue;
                    };

                    // dbg!((row, col));

                    if board[row as usize][col as usize] == 1 {
                        live_neighbors += 1;
                    }
                }

                let is_alive = board[row][col] == 1;

                match live_neighbors {
                    // Remain alive if not overpopulated.
                    2 | 3 if is_alive => ans[row][col] = 1,
                    // Be birthed by reproduction.
                    3 if !is_alive => ans[row][col] = 1,
                    // Otherwise, stay dead or die by overpopulation.
                    _ => (),
                }
            }
        }

        *board = ans;
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
