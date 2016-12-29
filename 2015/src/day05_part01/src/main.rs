use std::env::args;
use std::fs::File;
use std::io::Read;

static VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
static VERBOTEN: [&'static str; 4] = ["ab", "cd", "pq", "xy"];

fn main() {
    let mut count = 0;
    let lines = read_lines();
    for line in lines {
        if count_vowels(&line) >= 3 && contains_double(&line) && !contains_verboten(&line) {
            count += 1;
        }
    }
    println!("Nice string count: {}", count);
}

fn count_vowels(s: &str) -> usize {
    s.chars().filter(|c| VOWELS.contains(c)).count()
}

fn contains_double(s: &str) -> bool {
    let mut previous = s.chars().next().expect("Empty line");
    for c in s.chars().skip(1) {
        if previous == c {
            return true;
        }
        previous = c;
    }
    false
}

fn contains_verboten(s: &str) -> bool {
    let mut iter = s.chars();
    let mut previous = iter.next().expect("Empty line");
    while let Some(next) = iter.next() {
        if VERBOTEN.contains(&format!("{}{}", previous, next).as_str()) {
            return true;
        }
        previous = next;
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
