use std::env::args;
use std::fs::File;
use std::io::Read;

fn main() {
    let filename = read_filename();
    let lines = load_file(&filename);
    let discs: Vec<Disc> = lines.iter().map(|s| Disc::parse(s)).collect();
    println!("Loaded: {} discs", discs.len());

    let mut time = 0;
    'outer: loop {
        for (offset, disc) in discs.iter().enumerate() {
            if disc.rotate(time + offset + 1) != 0 {
                time += 1;
                continue 'outer;
            } else {
            }
        }
        break;
    }
    println!("Time: {}", time);
}

struct Disc {
    position: usize,
    position_count: usize,
}

impl Disc {
    fn parse(s: &str) -> Disc {
        let position_count = s.split_whitespace().nth(3).unwrap().parse().unwrap();
        let position = s.split_whitespace().last().unwrap().trim_matches('.').parse().unwrap();
        Disc {
            position: position,
            position_count: position_count,
        }
    }

    fn rotate(&self, n: usize) -> usize {
        let new = self.position + n;
        if new >= self.position_count {
            return new % self.position_count;
        }
        new
    }
}

fn read_filename() -> String {
    args().skip(1).next().unwrap()
}

fn load_file(filename: &str) -> Vec<String> {
    let mut file = File::open(filename).expect(&format!("Unable to open: {}", filename));
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();
    buf.lines().map(|s| s.to_owned()).collect()
}
