struct Solution;

impl Solution {
    /// Attempts to divide players into `n / 2` teams of equal skill, where
    /// `n` is equal to the number of participating players, and each team
    /// consists of two players.
    ///
    /// Returns the cumulative "chemistry", equal to the sum of the products
    /// of each teams' skills.
    ///
    /// # Parameters
    ///
    /// + `skill`: even-length collection of player skill levels.
    #[allow(dead_code, clippy::needless_pass_by_value)]
    pub fn divide_players(skill: Vec<i32>) -> i64 {
        let mut skill = skill;
        skill.sort_unstable();

        let mut left_idx: usize = 0;
        let mut right_idx: usize = skill.len() - 1;

        let mut chemistry: i64 = 0;

        let mut target_sum: Option<i32> = None;

        while left_idx < right_idx {
            let member_1 = skill.get(left_idx).unwrap();
            let member_2 = skill.get(right_idx).unwrap();

            let member_sum = member_1 + member_2;

            if target_sum.is_none() {
                target_sum = Some(member_sum);
            } else if let Some(sum) = target_sum && sum != member_sum {
                return -1;
            }

            chemistry += i64::from(member_1 * member_2);

            left_idx += 1;
            right_idx -= 1;
        }

        chemistry
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(Solution::divide_players(vec![3, 2, 5, 1, 3, 4]), 22);
    }
}
