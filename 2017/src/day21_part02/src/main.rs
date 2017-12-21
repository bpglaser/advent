extern crate day21_part02;

use std::env::args;
use std::fs::File;
use std::io::Read;

use day21_part02::do_puzzle;

const DEFAULT_ITERATION_COUNT: u32 = 5;

fn main() {
    let path = args().nth(1).expect("input path");
    let iteration_count = args()
        .nth(2)
        .and_then(|s| s.parse().ok())
        .unwrap_or(DEFAULT_ITERATION_COUNT);

    let input = load_input(&path);
    let answer = do_puzzle(&input, iteration_count);
    println!("Lit pixels: {}", answer);
}

fn load_input(path: &str) -> String {
    let mut file = File::open(path).unwrap();
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();
    buf
}
