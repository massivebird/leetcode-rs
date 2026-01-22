struct Solution;

impl Solution {
    #[allow(dead_code, clippy::needless_pass_by_ref_mut, clippy::ptr_arg)]
    pub fn game_of_life(board: &mut Vec<Vec<i32>>) {
        let width: usize = board[0].len();

        let mut ans = vec![vec![0; width]; board.len()];

        for row in 0..board.len() {
            for col in 0..width {
                let mut live_neighbors = 0u32;

                // Modifiers to (row, col) to find this cell's eight neighbors.
                for tr in [
                    // Top three
                    (-1, -1),
                    (-1, 0),
                    (-1, 1),
                    // Middle two
                    (0, -1),
                    (0, 1),
                    // Bottom three
                    (1, -1),
                    (1, 0),
                    (1, 1),
                ] {
                    // Compute this neighbor's hypothetical location.
                    //
                    // I convert to a signed type to simplify subtraction ops.
                    let row = isize::try_from(row).unwrap() + tr.0;
                    let col = isize::try_from(col).unwrap() + tr.1;

                    // Verify (row, col) is within board bounds.
                    let Some(row) = usize::try_from(row).ok().filter(|v| *v < board.len()) else {
                        continue;
                    };

                    let Some(col) = usize::try_from(col).ok().filter(|v| *v < width) else {
                        continue;
                    };

                    if board[row][col] == 1 {
                        live_neighbors += 1;
                    }
                }

                let is_alive = board[row][col] == 1;

                match live_neighbors {
                    // Stay alive if not overpopulated.
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
