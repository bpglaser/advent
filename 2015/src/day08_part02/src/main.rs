use std::env::args;
use std::fs::File;
use std::io::Read;

fn main() {
    let lines = load_lines();

    let mut total_actual_len = 0;
    let mut total_parsed_len = 0;

    for line in lines.iter() {
        let (actual_len, parsed_len) = get_lengths(&line);
        println!("{} => {} {}", line, actual_len, parsed_len);
        total_actual_len += actual_len;
        total_parsed_len += parsed_len;
    }

    let answer = total_parsed_len - total_actual_len;
    println!("Answer: {}", answer);
}

fn get_lengths(s: &str) -> (usize, usize) {
    let actual_len = s.len();
    let mut parsed_len = 2;

    let mut chars = s.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            '\\' | '\"' => parsed_len += 2,
            _ => parsed_len += 1,
        }
    }

    (actual_len, parsed_len)
}

fn load_lines() -> Vec<String> {
    let mut file = File::open(args().nth(1).unwrap()).unwrap();
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();
    buf.lines().map(|s| s.trim().to_owned()).collect()
}
