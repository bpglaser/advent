use std::collections::HashMap;
use std::env::args;
use std::fs::File;
use std::io::{BufRead, BufReader};

type Rules = HashMap<(char, char), char>;
type Counts = HashMap<(char, char), usize>;

fn parse_rule(s: String) -> ((char, char), char) {
    let words: Vec<_> = s.split_ascii_whitespace().collect();
    let mut pair = words[0].chars();
    let mid = words[2].chars().next().unwrap();
    ((pair.next().unwrap(), pair.next().unwrap()), mid)
}

fn read_input(path: &str) -> (String, Rules) {
    let f = File::open(path).unwrap();
    let reader = BufReader::new(f);
    let lines: Vec<_> = reader.lines().collect();
    let template = lines[0].as_ref().unwrap().to_owned();
    let mut rules = HashMap::new();
    for line in lines.into_iter().skip(2) {
        let (pair, mid) = parse_rule(line.unwrap());
        rules.insert(pair, mid);
    }
    (template, rules)
}

fn do_count(template: &str, pairs: &Counts) -> (usize, usize) {
    let mut counts: HashMap<char, usize> = HashMap::new();
    for (&(a, _), i) in pairs {
        *counts.entry(a).or_default() += i;
    }
    *counts.entry(template.chars().last().unwrap()).or_default() += 1;
    let max = counts.iter().max_by_key(|(_, i)| *i).unwrap();
    let min = counts.iter().min_by_key(|(_, i)| *i).unwrap();
    (*min.1, *max.1)
}

fn count_pairs(chars: impl Iterator<Item = char>) -> Counts {
    let mut iter = chars.peekable();
    let mut result = HashMap::new();
    while let Some(c) = iter.next() {
        if let Some(d) = iter.peek() {
            *result.entry((c, *d)).or_default() += 1;
        }
    }
    result
}

fn solve(template: &str, rules: &Rules, iter_count: usize) -> Counts {
    let mut counts = count_pairs(template.chars());
    for _ in 0..iter_count {
        let mut next = HashMap::new();
        for (&(a, b), i) in &counts {
            let c = rules[&(a, b)];
            *next.entry((a, c)).or_default() += i;
            *next.entry((c, b)).or_default() += i;
        }
        counts = next;
    }
    counts
}

fn main() {
    let path = args().nth(1).unwrap();
    let iter_count = args().nth(2).unwrap().parse().unwrap();
    let (template, rules) = read_input(&path);
    let result = solve(&template, &rules, iter_count);
    let (min, max) = do_count(&template, &result);
    println!("{}", max - min);
}
