use std::{
    env::args,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> Result<(), Box<dyn Error>> {
    let path = args().skip(1).next().ok_or("provide an path")?;
    let f = File::open(path)?;
    let mut sum = 0;
    for line in BufReader::new(f).lines() {
        let line = line?;
        sum += calibration_value(&line);
    }
    println!("{sum}");
    Ok(())
}

fn calibration_value(s: &str) -> u32 {
    let mut nums = s.chars().filter_map(|c| c.to_digit(10));
    let i = nums.next().expect("each row should have the first number");
    let j = nums.last().unwrap_or(i);
    i * 10 + j
}
