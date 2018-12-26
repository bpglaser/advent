use std::env::args;
use std::fmt;
use std::fs::File;
use std::io::Read;
use std::str::FromStr;

fn main() {
    let path = args().nth(1).expect("input path");
    let nanobots = parse_input(&path);

    let candidates: Vec<_> = nanobots
        .iter()
        .flat_map(|nanobot| {
            vec![
                (nanobot.x, nanobot.y, nanobot.z - nanobot.r),
                (nanobot.x, nanobot.y, nanobot.z + nanobot.r),
                (nanobot.x, nanobot.y - nanobot.r, nanobot.z),
                (nanobot.x, nanobot.y + nanobot.r, nanobot.z),
                (nanobot.x - nanobot.r, nanobot.y, nanobot.z),
                (nanobot.x + nanobot.r, nanobot.y, nanobot.z),
            ]
        })
        .map(|pos| (pos, test_pos(pos, &nanobots)))
        .collect();

    let mut max_n = *candidates.iter().map(|(_pos, n)| n).max().unwrap();
    let best: Vec<_> = candidates.iter().filter(|&&(_pos, n)| n == max_n).collect();

    let mut best_pos = best[0].0;

    'step: loop {
        for candidate in adjacent_candidates(best_pos) {
            let n = test_pos(candidate, &nanobots);
            if n >= max_n {
                if n > max_n {
                    max_n = n;
                }
                best_pos = candidate;
                continue 'step;
            }
        }
        break;
    }

    println!("{}", best_pos.0.abs() + best_pos.1.abs() + best_pos.2.abs());
}

fn adjacent_candidates(pos: (i32, i32, i32)) -> Vec<(i32, i32, i32)> {
    let mut result = Vec::new();
    if pos.0 != 0 {
        result.push((pos.0 - pos.0.signum(), pos.1, pos.2));
    }
    if pos.1 != 0 {
        result.push((pos.0, pos.1 - pos.1.signum(), pos.2));
    }
    if pos.2 != 0 {
        result.push((pos.0, pos.1, pos.2 - pos.2.signum()));
    }
    if pos.0 != 0 && pos.1 != 0 {
        result.push((pos.0 - pos.0.signum(), pos.1 - pos.1.signum(), pos.2));
    }
    if pos.1 != 0 && pos.2 != 0 {
        result.push((pos.0, pos.1 - pos.1.signum(), pos.2 - pos.2.signum()));
    }
    if pos.2 != 0 && pos.0 != 0 {
        result.push((pos.0 - pos.0.signum(), pos.1, pos.2 - pos.2.signum()));
    }
    result
}

fn test_pos(pos: (i32, i32, i32), nanobots: &[NanoBot]) -> usize {
    nanobots
        .iter()
        .filter(|nanobot| {
            let (dx, dy, dz) = (nanobot.x - pos.0, nanobot.y - pos.1, nanobot.z - pos.2);
            let dist = dx.abs() + dy.abs() + dz.abs();
            dist <= nanobot.r
        })
        .count()
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct NanoBot {
    x: i32,
    y: i32,
    z: i32,
    r: i32,
}

fn parse_input(path: &str) -> Vec<NanoBot> {
    let mut buf = String::new();
    let mut file = File::open(path).unwrap();
    file.read_to_string(&mut buf).unwrap();
    buf.lines().map(|line| line.parse().unwrap()).collect()
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
