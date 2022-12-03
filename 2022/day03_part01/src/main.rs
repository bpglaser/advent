use std::{collections::HashSet, env::args, fs::read_to_string};

struct Rucksack<'a>(&'a str);

impl<'a> Rucksack<'a> {
    fn sides(&self) -> (&str, &str) {
        let mid = self.0.len() / 2;
        self.0.split_at(mid)
    }

    fn in_both_sides(&self) -> Option<char> {
        let (l, r) = self.sides();
        let l: HashSet<_> = l.chars().collect();
        r.chars().find(|c| l.contains(c))
    }

    fn priority(&self) -> Option<u32> {
        self.in_both_sides().and_then(item_priority)
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
    let total: u32 = s
        .lines()
        .map(|line| {
            let sack = Rucksack(line);
            sack.priority().expect(&format!("malformed line: {}", line))
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
        assert_eq!(solve(&given), "157");
    }
}
