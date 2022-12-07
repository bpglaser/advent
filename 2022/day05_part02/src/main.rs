use std::{cmp, env::args, fs::read_to_string};

use regex::Regex;

#[derive(Debug)]
struct Stacks {
    stacks: Vec<Vec<char>>,
}

impl Stacks {
    fn shift(&mut self, count: usize, from: usize, to: usize) {
        let from_len = self.stacks[from].len();
        let count = cmp::min(count, from_len);
        let i = from_len - count;
        let removed_chunk: Vec<_> = self.stacks[from].drain(i..).collect();
        self.stacks[to].extend(removed_chunk.into_iter());
    }

    fn get_tops(&self) -> String {
        self.stacks
            .iter()
            .filter_map(|stack| stack.last().cloned())
            .collect()
    }
}

fn parse_stack_line(s: &str) -> Vec<Option<char>> {
    let chars: Vec<_> = s.chars().collect();
    chars
        .chunks(4)
        .map(|chunk| {
            if chunk[0].is_whitespace() {
                None
            } else {
                Some(chunk[1])
            }
        })
        .collect()
}

fn parse_stacks(s: &str) -> (Stacks, usize) {
    let horizontal_stacks: Vec<_> = s
        .lines()
        .map(|line| parse_stack_line(line))
        .take_while(|stack_line| stack_line.iter().any(|slot| slot.is_some()))
        .collect();
    let lines_consumed = horizontal_stacks.len() + 1;
    let col_count = horizontal_stacks[0].len();

    let mut columns = vec![vec![]; col_count];
    for row in horizontal_stacks.into_iter().rev() {
        for (i, entry) in row.into_iter().enumerate() {
            if let Some(entry) = entry {
                columns[i].push(entry);
            }
        }
    }

    let stacks = Stacks { stacks: columns };
    (stacks, lines_consumed)
}

fn parse_instruction(s: &str) -> (usize, usize, usize) {
    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    let caps = re.captures(s).unwrap();
    (
        caps[1].parse().unwrap(),
        // Columns are 1 indexed and, uh, lets not.
        caps[2].parse::<usize>().unwrap() - 1,
        caps[3].parse::<usize>().unwrap() - 1,
    )
}

fn solve(s: &str) -> String {
    let (mut stacks, lines_consumed) = parse_stacks(s);
    let instructions: Vec<_> = s
        .lines()
        .skip(lines_consumed + 1)
        .map(|line| parse_instruction(line))
        .collect();
    for (count, from, to) in instructions {
        stacks.shift(count, from, to);
    }
    stacks.get_tops()
}

fn input() -> String {
    let path = args().nth(1).unwrap();
    read_to_string(path).unwrap()
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
        let given = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
        assert_eq!(solve(&given), "MCD");
    }
}
