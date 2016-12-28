use std::env::args;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut sum = 0;
    for line in read_lines() {
        let mut dimensions: Vec<isize> = line.split('x').map(|s| s.parse().unwrap()).collect();
        dimensions.sort();
        let l = dimensions[0];
        let w = dimensions[1];
        let h = dimensions[2];
        sum += (3 * l * w) + (2 * w * h) + (2 * h * l);
    }
    println!("Total square footage: {}", sum);
}

fn read_lines() -> Vec<String> {
    let path = args().skip(1).next().unwrap();
    let file = File::open(path).unwrap();
    let file = BufReader::new(file);
    file.lines().map(|line| line.unwrap()).collect()
}
