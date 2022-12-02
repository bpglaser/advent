use std::{env::args, fs::read_to_string, mem};

fn input() -> String {
    let path = args().skip(1).next().unwrap();
    read_to_string(path).unwrap()
}

fn parse(s: &str) -> Vec<Vec<usize>> {
    let mut all = vec![];
    let mut elf = vec![];
    for line in s.lines() {
        if line == "" {
            let mut tmp = vec![];
            mem::swap(&mut tmp, &mut elf);
            all.push(tmp);
        } else {
            elf.push(line.parse::<usize>().unwrap());
        }
    }
    all
}

fn solve(s: &str) -> String {
    let elves = parse(s);
    let max: usize = elves.into_iter().map(|elf| elf.iter().sum()).max().unwrap();
    format!("{}", max)
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
        let given = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";
        assert_eq!(solve(given), "24000".to_owned());
    }
}
