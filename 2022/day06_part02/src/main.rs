use std::{collections::HashSet, env::args, fs::read_to_string};

const WINDOW_SIZE: usize = 14;

fn solve(s: &str) -> String {
    let chars: Vec<_> = s.chars().collect();
    let mut unique: HashSet<char> = HashSet::with_capacity(4);
    for i in 0..=chars.len() - WINDOW_SIZE {
        unique.extend(&chars[i..i + WINDOW_SIZE]);
        if unique.len() == WINDOW_SIZE {
            return format!("{}", i + WINDOW_SIZE);
        }
        unique.clear();
    }
    panic!("ruh roh");
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
        let cases = [
            ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", "19"),
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", "23"),
            ("nppdvjthqldpwncqszvftbrmjlhg", "23"),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", "29"),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", "26"),
        ];
        for (given, expected) in cases {
            assert_eq!(solve(&given), expected, "solve({})", given);
        }
    }
}
