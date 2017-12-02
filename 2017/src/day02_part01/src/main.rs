use std::fs::File;
use std::io::Read;
use std::env::args;

fn main() {
    let path = args().nth(1).unwrap();
    let input = load_input(&path);

    let mut sum = 0;

    for line in input.lines() {
        let mut iterator = line.split_whitespace().map(|s| s.parse::<i32>().unwrap());
        let mut min = iterator.next().unwrap();
        let mut max = min;
        for n in iterator {
            if n > max {
                max = n;
            } else if n < min {
                min = n;
            }
        }
        sum += max - min;
    }

    println!("captcha: {}", sum);
}

fn load_input(path: &str) -> String {
    let mut file = File::open(path).unwrap();
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();
    buf
}