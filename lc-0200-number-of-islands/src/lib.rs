//! [Challenge](https://leetcode.com/problems/number-of-islands?envType=study-plan-v2&envId=top-interview-150)
//!
//! Given an m x n 2D binary grid grid which represents a map of '1's (land)
//! and '0's (water), return the number of islands.

struct Solution;

impl Solution {
    #[allow(dead_code, clippy::needless_pass_by_value)]
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let height = grid.len();
        let width = grid[0].len();

        // `[i][j] == true` means that the land at `grid[i][j]` has
        // been traversed, i.e. has been counted as (part of) an island.
        let mut chartered = vec![vec![false; width]; height];

        let mut islands = 0;

        for (i, row) in grid.iter().enumerate() {
            for (j, char) in row.iter().enumerate() {
                if *char == '1' && !chartered[i][j] {
                    Self::bfs(i, j, &grid, &mut chartered);
                    islands += 1;
                }
            }
        }

        islands
    }

    /// Traverses an island using BFS. Marks each traversed land space in
    /// the `chartered` vector.
    fn bfs(i: usize, j: usize, grid: &[Vec<char>], chartered: &mut [Vec<bool>]) {
        use std::collections::VecDeque;

        let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
        queue.push_back((i, j));

        while let Some((i, j)) = queue.pop_front() {
            // Continue only if this space exists and is land.
            if grid
                .get(i)
                .is_none_or(|row| row.get(j).is_none_or(|c| *c != '1'))
            {
                continue;
            }

            // Ignore this land space if it's already chartered.
            if chartered[i][j] {
                continue;
            }

            chartered[i][j] = true;

            // Queue adjacent spaces.
            queue.push_back((i.saturating_sub(1), j)); // North
            queue.push_back((i + 1, j)); // South
            queue.push_back((i, j + 1)); // East
            queue.push_back((i, j.saturating_sub(1))); // West
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_0() {
        let grid = vec![
            vec!['1', '1', '1', '1', '0'],
            vec!['1', '1', '0', '1', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '0', '0', '0'],
        ];

        assert_eq!(Solution::num_islands(grid), 1);
    }

    #[test]
    fn case_1() {
        let grid = vec![
            vec!['1', '1', '0', '0', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '1', '0', '0'],
            vec!['0', '0', '0', '1', '1'],
        ];

        assert_eq!(Solution::num_islands(grid), 3);
    }

    #[test]
    fn case_2() {
        let grid = vec![
            vec!['0', '1', '1', '0', '0'],
            vec!['0', '1', '1', '1', '0'],
            vec!['1', '1', '1', '0', '1'],
            vec!['0', '0', '0', '0', '1'],
        ];

        assert_eq!(Solution::num_islands(grid), 2);
    }

    #[test]
    fn case_3() {
        let grid = vec![
            vec!['1', '0', '1', '1', '1'],
            vec!['1', '0', '1', '0', '1'],
            vec!['1', '1', '1', '0', '1'],
        ];

        assert_eq!(Solution::num_islands(grid), 1);
    }
}
