use std::{
    cmp::max,
    env::args,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

type Result<T, E = Box<dyn Error>> = std::result::Result<T, E>;

type Draw = [u32; 3];

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
        let right = &line[i + 1..];
        let fewest_cubes = right
            .split(";")
            .map(parse_draw)
            .fold([0; 3], |acc, draw| max_draws(acc, draw));
        let power: u32 = fewest_cubes.iter().product();
        tot += power;
    }
    println!("{tot}");

    Ok(())
}

fn parse_draw(s: &str) -> Draw {
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

fn max_draws(mut a: Draw, b: Draw) -> Draw {
    a[0] = max(a[0], b[0]);
    a[1] = max(a[1], b[1]);
    a[2] = max(a[2], b[2]);
    a
}
