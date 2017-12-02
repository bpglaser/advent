use std::fs::File;
use std::io::Read;
use std::env::args;

fn main() {
    let path = args().nth(1).unwrap();
    let input = load_input(&path);

    let mut sum = 0;

    for line in input.lines() {
        let mut numbers: Vec<i32> = line.split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();

        numbers.sort_by(|a, b| b.cmp(a));

        for (i, n) in numbers.iter().enumerate() {
            for m in numbers.iter().skip(i + 1) {
                if n % m == 0 {
                    sum += n / m;
                }
            }
        }
    }

    println!("captcha: {}", sum);
}

fn load_input(path: &str) -> String {
    let mut file = File::open(path).unwrap();
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();
    buf
}
