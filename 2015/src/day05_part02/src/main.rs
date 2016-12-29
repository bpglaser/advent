use std::env::args;
use std::fs::File;
use std::io::Read;

fn main() {
    let mut count = 0;
    let lines = read_lines();
    for line in lines {
        if contains_double_pair(&line) {
            if contains_sandwich_triple(&line) {
                count += 1;
            }
        }
    }
    println!("Nice string count: {}", count);
}

fn contains_double_pair(s: &str) -> bool {
    for (i, a) in s.chars().enumerate() {
        if let Some(b) = s.chars().nth(i + 1) {
            for (j, c) in s.chars().skip(i + 2).enumerate() {
                if let Some(d) = s.chars().nth(i + j + 3) {
                    if (a, b) == (c, d) {
                        return true;
                    }
                } else {
                    break;
                }
            }
        } else {
            break;
        }
    }
    false
}

fn contains_sandwich_triple(s: &str) -> bool {
    for (n, a) in s.chars().enumerate() {
        if let Some(c) = s.chars().nth(n + 2) {
            if a == c {
                return true;
            }
        }
    }
    false
}

fn read_lines() -> Vec<String> {
    let path = args().skip(1).next().expect("Invalid args");
    let mut file = File::open(path).expect("Error opening file");
    let mut buf = String::new();
    file.read_to_string(&mut buf).expect("Error reading file");
    buf.lines().map(|s| s.trim().to_owned()).collect()
}
