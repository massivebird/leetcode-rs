#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn divide_players(skill: Vec<i32>) -> i64 {
        let mut skill = skill;
        skill.sort();

        let mut teams: Vec<Vec<i32>> = Vec::new();

        for i in 0..skill.len() / 2 {
            let member_1 = skill.get(i).unwrap();
            let member_2 = skill.get(skill.len() - 1 - i).unwrap();

            teams.push(vec![*member_1, *member_2]);
        }

        let target_sum: i32 = teams.first().unwrap().iter().sum();

        for team in teams.iter().skip(1) {
            if team.iter().sum::<i32>() != target_sum {
                return -1;
            }
        }

        teams.iter().fold(0i64, |acc, team| {
            acc + (team.first().unwrap() * team.get(1).unwrap()) as i64
        })
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
