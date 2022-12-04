use std::{env::args, fs::read_to_string};

use regex::Regex;

type Range = (u32, u32);
type Pair = (Range, Range);

fn extract(re: &Regex, line: &str) -> Pair {
    let groups: Vec<_> = re
        .captures(line)
        .unwrap()
        .iter()
        .skip(1)
        .map(|cap| cap.unwrap().as_str().parse().unwrap())
        .collect();
    assert!(groups.len() == 4);
    ((groups[0], groups[1]), (groups[2], groups[3]))
}

fn is_fully_contained((a, b): &Pair) -> bool {
    (a.0 >= b.0 && a.1 <= b.1) || (b.0 >= a.0 && b.1 <= a.1)
}

fn input() -> String {
    let path = args().nth(1).expect("input path");
    read_to_string(path).expect("reading input")
}

fn solve(s: &str) -> String {
    let re = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();
    let i = s
        .lines()
        .map(|line| extract(&re, line))
        .filter(is_fully_contained)
        .count();
    i.to_string()
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
        let given = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
        let actual = solve(&given);
        assert_eq!(actual, "2");
    }
}
