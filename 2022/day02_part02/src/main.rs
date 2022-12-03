use std::{env::args, fs::read_to_string};

use Outcome::*;
use Sign::*;

const WIN_SCORE: usize = 6;
const LOSE_SCORE: usize = 0;
const TIE_SCORE: usize = 3;

enum Outcome {
    Win,
    Lose,
    Tie,
}

impl Outcome {
    fn parse(s: &str) -> Option<Self> {
        match s {
            "X" => Some(Lose),
            "Y" => Some(Tie),
            "Z" => Some(Win),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Sign {
    Rock,
    Paper,
    Scissors,
}

impl Sign {
    fn parse(s: &str) -> Option<Self> {
        match s {
            "A" => Some(Rock),
            "B" => Some(Paper),
            "C" => Some(Scissors),
            _ => None,
        }
    }

    fn score(&self) -> usize {
        match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }

    fn with(&self, outcome: Outcome) -> Self {
        match outcome {
            Win if self == &Rock => Paper,
            Win if self == &Paper => Scissors,
            Win if self == &Scissors => Rock,
            Lose if self == &Rock => Scissors,
            Lose if self == &Paper => Rock,
            Lose if self == &Scissors => Paper,
            Tie => *self,
            _ => unreachable!(),
        }
    }
}

fn input() -> String {
    let path = args().nth(1).unwrap();
    read_to_string(path).unwrap()
}

fn parse_row(s: &str) -> Option<(Sign, Outcome)> {
    let mut split = s.split_whitespace();
    split
        .next()
        .and_then(Sign::parse)
        .and_then(|l| split.next().and_then(Outcome::parse).map(|r| (l, r)))
}

fn score((opponent, player): (Sign, Sign)) -> usize {
    match (opponent, player) {
        // Wins
        (Rock, Paper) | (Paper, Scissors) | (Scissors, Rock) => player.score() + WIN_SCORE,
        // Losses
        (Rock, Scissors) | (Paper, Rock) | (Scissors, Paper) => player.score() + LOSE_SCORE,
        // Ties
        (Rock, Rock) | (Paper, Paper) | (Scissors, Scissors) => player.score() + TIE_SCORE,
    }
}

fn solve(s: &str) -> String {
    let rows: Option<Vec<_>> = s.lines().map(parse_row).collect();
    let rows = rows.expect("malformed input");
    let total: usize = rows
        .into_iter()
        .map(|(opponent, outcome)| score((opponent, opponent.with(outcome))))
        .sum();
    format!("{}", total)
}

fn main() {
    let s = input();
    let res = solve(&s);
    println!("{}", res);
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test_given() {
        let given = "A Y
B X
C Z";
        let actual = solve(&given);
        let expected = "12";
        assert_eq!(actual, expected);
    }
}
