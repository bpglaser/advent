use std::{collections::HashSet, env::args, fs::read_to_string};

const GROUP_SIZE: usize = 3;

struct Rucksack<'a>(&'a str);

impl<'a> Rucksack<'a> {
    fn unique(&self) -> HashSet<char> {
        self.0.chars().collect()
    }
}

fn item_priority(c: char) -> Option<u32> {
    match c {
        'a'..='z' => Some(c as u32 - 'a' as u32 + 1),
        'A'..='Z' => Some(c as u32 - 'A' as u32 + 27),
        _ => None,
    }
}

fn input() -> String {
    let path = args().nth(1).expect("missing path arg");
    read_to_string(path).expect("error reading input")
}

fn solve(s: &str) -> String {
    let sacks: Vec<_> = s.lines().map(Rucksack).collect();
    let total: u32 = sacks
        .chunks(GROUP_SIZE)
        .map(|chunk| {
            let (a, b, c) = (&chunk[0], &chunk[1], &chunk[2]);
            let ab: HashSet<_> = a.unique().intersection(&b.unique()).cloned().collect();
            let c = c.unique();
            let shared: Vec<_> = ab.intersection(&c).collect();
            assert!(shared.len() == 1);
            item_priority(*shared[0]).expect("malformed input")
        })
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
        let given = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
        assert_eq!(solve(&given), "70");
    }
}
