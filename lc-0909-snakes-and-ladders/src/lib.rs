mod tests;

struct Solution;

impl Solution {
    #[allow(dead_code, clippy::needless_pass_by_value)]
    pub fn snakes_and_ladders(board: Vec<Vec<i32>>) -> i32 {
        let mut b = Vec::new();

        for (i, row) in board.iter().rev().enumerate() {
            if i.is_multiple_of(2) {
                for s in row {
                    b.push(s);
                }
            } else {
                for s in row.iter().rev() {
                    b.push(s);
                }
            }
        }

        // dbg!(&b);

        let n = board.len();

        let mut visited: Vec<Option<i32>> = vec![None; n * n];
        Self::bfs(&b, 0, &mut visited, 1, 6, false);
        dbg!(&visited);
        // Self::_dbg_print(&visited, n);

        visited[(n * n) - 1].unwrap()
    }

    fn bfs(
        board: &[&i32],
        i: usize,
        visited: &mut [Option<i32>],
        turn: i32,
        dice_rem: usize,
        snuck: bool,
    ) {
        if turn == 1 {
            // dbg!((i, step));
        }
        let Some(vst) = visited.get_mut(i) else {
            return;
        };

        if vst.is_some_and(|v| v < turn) {
            return;
        }

        *vst = Some(turn);

        let (next_turn, next_dice, snuck) = if dice_rem == 0 {
            (turn + 1, 6, false)
        } else {
            (turn, dice_rem - 1, snuck)
        };

        // Self::bfs(board, i.saturating_sub(1), visited, next_step, next_dice);

        if *board[i] > -1 && !snuck {
            Self::bfs(
                board,
                (*board[i] - 1) as usize,
                visited,
                turn,
                dice_rem,
                true,
            );
        } else {
            Self::bfs(
                board,
                i.saturating_add(1),
                visited,
                next_turn,
                next_dice,
                snuck,
            );
        }
    }

    fn _dbg_print(board: &[Option<i32>], n: usize) {
        for (i, v) in board.iter().enumerate() {
            print!("{v:3?}");
            if i > 0 && (i + 1).is_multiple_of(n) {
                println!();
            }
        }
    }
}
