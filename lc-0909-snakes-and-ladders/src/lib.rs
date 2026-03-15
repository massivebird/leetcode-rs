//! <https://leetcode.com/problems/snakes-and-ladders/description/?envType=study-plan-v2&envId=top-interview-150>
//!
//! You are given an n x n integer matrix board where the cells are labeled
//! from 1 to n*n in a Boustrophedon style starting from the bottom left of
//! the board (i.e. `board[n - 1][0]`) and alternating direction each row.
//!
//! Return the minimum number of turns required to reach the end, or
//! -1 if it can't be done.
//!
//! MB: this question is TERRIBLY written. Here's an excerpt of slop:
//!
//! > "Note that you only take a snake or ladder at most once per dice roll.
//! > If the destination to a snake or ladder is the start of another snake or
//! > ladder, you do not follow the subsequent snake or ladder."
//!
//! Does this mean that taking a snake or ladder forces me to roll another die?
//! No thanks to the unclear language here, I eventually figured out that the
//! answer is yes.
//!
//! Anyway. My recursive approach blew the stack so here's a standard queue BFS.

mod tests;

struct Solution;

impl Solution {
    #[allow(dead_code, clippy::needless_pass_by_value)]
    pub fn snakes_and_ladders(board: Vec<Vec<i32>>) -> i32 {
        let n = board.len();

        // Reconstruct the 2D board into 1D.
        let board: Vec<&i32> = {
            let mut tmp = Vec::new();

            for (i, row) in board.iter().rev().enumerate() {
                // mind the boustrophedon layout
                if i.is_multiple_of(2) {
                    for s in row {
                        tmp.push(s);
                    }
                } else {
                    for s in row.iter().rev() {
                        tmp.push(s);
                    }
                }
            }

            tmp
        };

        let mut visited: Vec<Option<i32>> = vec![None; n * n];

        // (index, turn#)
        let mut queue: std::collections::VecDeque<(usize, i32)> =
            std::collections::VecDeque::from(vec![(0, 0)]);

        while let Some((i, turn)) = queue.pop_front() {
            let vst = &mut visited[i];

            // Skup if this space has been reached in fewer turns.
            if vst.is_some_and(|v| v <= turn) {
                continue;
            }

            *vst = Some(turn);

            for roll in 1..=6 {
                // Roll until OOB.
                let Some(board_val) = board.get(i + roll) else {
                    break;
                };

                // If roll lands on a snake/ladder, take it.
                if **board_val > -1 {
                    #[allow(clippy::cast_sign_loss)]
                    queue.push_back(((*board_val - 1) as usize, turn + 1));
                } else {
                    queue.push_back((i + roll, turn + 1));
                }
            }
        }

        visited[(n * n) - 1].unwrap_or(-1)
    }
}
