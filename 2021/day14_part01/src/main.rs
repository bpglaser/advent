use std::collections::HashMap;
use std::env::args;
use std::fs::File;
use std::io::{BufRead, BufReader};

type Rules = HashMap<(char, char), char>;

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

fn step(s: &str, rules: &Rules) -> String {
    let mut result = String::new();
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        if let Some(d) = chars.peek() {
            result.push(c);
            if let Some(mid) = rules.get(&(c, *d)) {
                result.push(*mid);
            }
        } else {
            result.push(c);
        }
    }
    result
}

fn main() {
    let path = args().nth(1).unwrap();
    let count = args().nth(2).unwrap().parse().unwrap();
    let (mut template, rules) = read_input(&path);
    for _ in 0..count {
        template = step(&template, &rules);
    }
    let mut counts: HashMap<char, i32> = HashMap::new();
    for c in template.chars() {
        let i = counts.entry(c).or_default();
        *i += 1;
    }
    let max = counts.iter().max_by_key(|(_, i)| *i).unwrap();
    let min = counts.iter().min_by_key(|(_, i)| *i).unwrap();
    println!("{}", max.1 - min.1);
}
