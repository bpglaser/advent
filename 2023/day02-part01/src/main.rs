use std::{
    env::args,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

const BOUNDS: [u32; 3] = [12, 13, 14];

type Result<T, E = Box<dyn Error>> = std::result::Result<T, E>;

fn main() -> Result<()> {
    let path = args().skip(1).next().ok_or("provide input path")?;
    let f = File::open(&path)?;
    let reader = BufReader::new(f);

    let mut tot = 0;

    for line in reader.lines() {
        let line = line?;
        let (i, _) = line
            .char_indices()
            .find(|(_, c)| *c == ':')
            .ok_or("failed to find ':'")?;
        let left = &line[..i];
        let game_n: u32 = left
            .split_whitespace()
            .skip(1)
            .next()
            .ok_or("failed to find game number")?
            .parse()?;
        let right = &line[i + 1..];
        let all_valid = right.split(";").map(parse_draw).all(|draw| is_valid(&draw));
        if all_valid {
            tot += game_n;
        }
    }
    println!("{tot}");

    Ok(())
}

fn parse_draw(s: &str) -> [u32; 3] {
    let mut res = [0u32; 3];
    for s in s.split(',') {
        let split = s.trim().split_once(' ').unwrap();
        let i = match split.1 {
            "red" => 0,
            "green" => 1,
            "blue" => 2,
            _ => unimplemented!(),
        };
        res[i] = split.0.parse().unwrap();
    }
    res
}

fn is_valid(draw: &[u32; 3]) -> bool {
    draw.iter().zip(BOUNDS.iter()).all(|(a, b)| a <= b)
}
