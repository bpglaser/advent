extern crate itertools;

use itertools::Itertools;

use std::env::args;
use std::fs::File;
use std::io::Read;

fn main() {
    let path = get_path().expect("Invalid input");
    let input = load_input(&path).expect("Error reading file");
    let target_size = sum(&input) / 4;

    for i in 1..input.len() {
        let mut candidates = vec![];
        for combos in input.iter().cloned().combinations(i) {
            if sum(&combos) == target_size {
                candidates.push(product(&combos));
            }
        }
        if candidates.len() > 0 {
            println!("Answer: {}", candidates.iter().min().unwrap());
            return;
        }
    }
}

fn sum(s: &[usize]) -> usize {
    s.iter().sum()
}

fn product(s: &[usize]) -> u64 {
    s.iter().fold(1u64, |acc, &x| acc * x as u64)
}

fn get_path() -> Option<String> {
    args().skip(1).next().map(|s| s.to_owned())
}

fn load_input(path: &str) -> Result<Vec<usize>, std::io::Error> {
    let mut file = File::open(path)?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;
    Ok(buf.trim().lines().map(|s| s.parse().expect("Invalid number")).collect())
}