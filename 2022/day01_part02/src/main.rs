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
    if !elf.is_empty() {
        all.push(elf);
    }
    all
}

fn solve(s: &str) -> String {
    let elves = parse(s);
    let mut elf_totals: Vec<usize> = elves.iter().map(|elf| elf.iter().sum()).collect();
    elf_totals.sort();
    let last_three = &elf_totals[elf_totals.len() - 3..];
    let max_three: usize = last_three.iter().sum();
    format!("{}", max_three)
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
        assert_eq!(solve(given), "45000".to_owned());
    }
}
