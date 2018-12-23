use std::env::args;
use std::fmt;
use std::fs::File;
use std::io::Read;
use std::str::FromStr;

fn main() {
    let path = args().nth(1).expect("input path");
    let nanobots = parse_input(&path);

    let biggest = nanobots.iter().max_by_key(|bot| bot.r).unwrap();
    println!("biggest: {}", biggest);

    let ans = nanobots
        .iter()
        .filter(|bot| biggest.range_contains(bot))
        .count();
    println!("ans {}", ans);
}

fn parse_input(path: &str) -> Vec<NanoBot> {
    let mut buf = String::new();
    let mut file = File::open(path).unwrap();
    file.read_to_string(&mut buf).unwrap();
    buf.lines().map(|line| line.parse().unwrap()).collect()
}

#[derive(Eq, PartialEq)]
struct NanoBot {
    x: i32,
    y: i32,
    z: i32,
    r: i32,
}

impl NanoBot {
    fn range_contains(&self, other: &Self) -> bool {
        let dx = (self.x - other.x).abs();
        let dy = (self.y - other.y).abs();
        let dz = (self.z - other.z).abs();
        dx + dy + dz <= self.r
    }
}

impl FromStr for NanoBot {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let raw_coords = s
            .chars()
            .skip(5)
            .take_while(|c| c != &'>')
            .collect::<String>();

        let mut coords = raw_coords.split(',').map(|s| s.parse().unwrap());

        let x = coords.next().unwrap();
        let y = coords.next().unwrap();
        let z = coords.next().unwrap();

        let r = s.split('=').last().unwrap().parse().unwrap();

        Ok(Self { x, y, z, r })
    }
}

impl fmt::Display for NanoBot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "pos=<{},{},{}>, r={}", self.x, self.y, self.z, self.r)
    }
}
