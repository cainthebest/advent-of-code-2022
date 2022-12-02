use crate::util::{load_data_file, AOC};

#[derive(Debug, Clone, Copy, PartialEq)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug)]
enum Outcome {
    Win,
    Lose,
    Tie,
}

impl Move {
    fn from_str(s: &str) -> Self {
        match s {
            "A" | "X" => Move::Rock,
            "B" | "Y" => Move::Paper,
            "C" | "Z" => Move::Scissors,
            _ => unreachable!(),
        }
    }

    fn from_index(i: usize) -> Self {
        match i {
            0 => Move::Rock,
            1 => Move::Paper,
            2 => Move::Scissors,
            _ => unreachable!(),
        }
    }

    fn derive(&self, win: bool) -> Self {
        Move::from_index((*self as usize + if win { 1 } else { 2 }) % 3)
    }
}

impl Outcome {
    fn to_score(&self) -> u16 {
        match self {
            Outcome::Lose => 0,
            Outcome::Tie => 3,
            Outcome::Win => 6,
        }
    }
}

fn score_round(other: Move, self_: Move) -> Outcome {
    if other == self_ {
        return Outcome::Tie;
    }

    if (other as u16 + 1) % 3 == self_ as u16 {
        return Outcome::Win;
    }

    Outcome::Lose
}

pub struct Day02;

impl AOC for Day02 {
    fn name(&self) -> &'static str {
        "(Day 2): Rock Paper Scissors"
    }

    fn year(&self) -> u16 {
        2022
    }

    fn part_a(&self) -> String {
        let raw: String = load_data_file(2022, 2);
        let mut score: u16 = 0;

        for (other, self_) in raw
            .lines()
            .filter(|x| !x.is_empty())
            .map(|x| x.split_once(" ").unwrap())
        {
            let other_move: Move = Move::from_str(other);
            let self_move: Move = Move::from_str(self_);

            score += self_move as u16 + 1;
            score += score_round(other_move, self_move).to_score();
        }

        score.to_string()
    }

    fn part_b(&self) -> String {
        let raw: String = load_data_file(2022, 2);
        let mut score: u16 = 0;

        for (other, self_) in raw
            .lines()
            .filter(|x| !x.is_empty())
            .map(|x| x.split_once(" ").unwrap())
        {
            let other_move: Move = Move::from_str(other);
            let self_move: Move = match self_ {
                "X" => other_move.derive(false),
                "Y" => other_move,
                "Z" => other_move.derive(true),
                _ => unreachable!(),
            };

            score += self_move as u16 + 1;
            score += score_round(other_move, self_move).to_score();
        }

        score.to_string()
    }
}
