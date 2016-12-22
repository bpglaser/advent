use std::env::args;
use std::fs::File;
use std::io::Read;

fn main() {
    let filename = get_input_filename();
    let mut ranges = load_ranges(&filename);
    ranges.sort_by(|a, b| a.low.cmp(&b.low));
    let mut answer: u32 = 0;
    'outer: loop {
        for range in ranges.iter() {
            if range.low > answer {
                break 'outer;
            }
            if range.contains(answer) {
                answer = range.high + 1;
                continue 'outer;
            }
        }
    }
    println!("Answer: {}", answer);
}

#[derive(Debug)]
struct Range {
    low: u32,
    high: u32,
}

impl Range {
    fn contains(&self, n: u32) -> bool {
        self.low <= n && self.high >= n
    }
}

impl Range {
    fn from_line(line: &str) -> Range {
        let line = line.trim();
        let a: String = line.chars().take_while(|c| c != &'-').collect();
        let b: String = line.chars().skip(a.len() + 1).collect();
        let a = a.parse().unwrap();
        let b = b.parse().unwrap();
        if a < b {
            Range { low: a, high: b }
        } else {
            Range { low: b, high: a }
        }
    }
}

fn load_ranges(path: &str) -> Vec<Range> {
    let mut buf = String::new();
    File::open(path).expect(&format!("Unable to open: {}", path))
        .read_to_string(&mut buf).expect(&format!("Error reading from: {}", path));
    buf.lines().map(Range::from_line).collect()
}

fn get_input_filename() -> String {
    args().skip(1).next().expect("Invalid input")
}
