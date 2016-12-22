use std::env::args;
use std::fs::File;
use std::io::Read;

fn main() {
    let filename = get_input_filename();
    let mut ranges = load_ranges(&filename);
    ranges.sort_by(|a, b| a.low.cmp(&b.low));
    let answers = get_answer(&ranges);
    println!("Valid IPs: {:?}", answers);
    println!("Answer: {}", answers.len());
}

fn get_answer(ranges: &Vec<Range>) -> Vec<u32> {
    let mut working_value = 0;
    let mut answers = vec![];
    for range in ranges.iter() {
        if range.low > working_value {
            answers.push(working_value);
            working_value += 1;
        }
        if range.contains(working_value) {
            if range.high == u32::max_value() {
                return answers;
            }
            working_value = range.high + 1;
        }
    }
    panic!()
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
