use std::{env::args, fs::read_to_string};

use Sign::*;

const WIN_SCORE: usize = 6;
const LOSE_SCORE: usize = 0;
const TIE_SCORE: usize = 3;

#[derive(Clone, Copy, PartialEq)]
enum Sign {
    Rock,
    Paper,
    Scissors,
}

impl Sign {
    fn score(&self) -> usize {
        match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }
}

fn input() -> String {
    let path = args().nth(1).unwrap();
    read_to_string(path).unwrap()
}

fn parse_sign(c: &str) -> Option<Sign> {
    match c {
        "A" | "X" => Some(Rock),
        "B" | "Y" => Some(Paper),
        "C" | "Z" => Some(Scissors),
        _ => None,
    }
}

fn parse_row(s: &str) -> Option<(Sign, Sign)> {
    let mut split = s.split_whitespace();
    match (
        split.next().and_then(parse_sign),
        split.next().and_then(parse_sign),
    ) {
        (Some(l), Some(r)) => Some((l, r)),
        _ => None,
    }
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
    let total: usize = rows.into_iter().map(score).sum();
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
        let expected = "15";
        assert_eq!(actual, expected);
    }
}
