use std::{
    env::args,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

const WORDS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn main() -> Result<(), Box<dyn Error>> {
    let path = args().skip(1).next().ok_or("provide an path")?;
    let f = File::open(path)?;
    let mut sum = 0;
    for line in BufReader::new(f).lines() {
        let line = line?;
        sum += calibration_value(&line);
    }
    println!("{sum}");
    Ok(())
}

fn calibration_value(s: &str) -> u32 {
    let forward: Vec<_> = WORDS
        .iter()
        .enumerate()
        .map(|(i, w)| (w.to_string(), i as u32 + 1))
        .collect();
    let i = find_first(s, forward.as_slice());
    let backwards: Vec<_> = WORDS
        .iter()
        .enumerate()
        .map(|(i, w)| (w.chars().rev().collect(), i as u32 + 1))
        .collect();
    let s: String = s.chars().rev().collect();
    let j = find_first(&s, backwards.as_slice());
    i.unwrap() * 10 + j.unwrap()
}

fn find_first(mut s: &str, words: &[(String, u32)]) -> Option<u32> {
    while s.len() > 0 {
        for (word, i) in words {
            if s.starts_with(word) {
                return Some(*i);
            }
        }
        if let Some(i) = try_read_u32(s) {
            return Some(i);
        }
        s = &s[1..];
    }
    None
}

fn try_read_u32(s: &str) -> Option<u32> {
    s.chars().next().and_then(|c| c.to_digit(10))
}
