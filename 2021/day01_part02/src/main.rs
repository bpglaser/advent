use std::env::args;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let path = args().skip(1).next().ok_or("not enough args")?;
    let mut f = File::open(path)?;
    let mut buf = vec![];
    f.read_to_end(&mut buf)?;
    let mut values = vec![];
    for line in buf.lines() {
        let line = line?;
        let value: i32 = line.parse()?;
        values.push(value);
    }
    let answer = values
        .windows(3)
        .map(|window| window[0] + window[1] + window[2])
        .collect::<Vec<_>>()
        .windows(2)
        .fold(0, |acc, xs| if xs[0] < xs[1] { acc + 1 } else { acc });
    println!("{:?}", answer);
    Ok(())
}
