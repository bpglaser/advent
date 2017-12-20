extern crate regex;

use std::env::args;
use std::fs::File;
use std::io::Read;

use regex::Regex;

mod vector3;
use vector3::*;

const LONG_TERM: i64 = 100_000_000;

fn main() {
    let path = args().nth(1).expect("valid input path");
    println!("Loading input...");
    let mut particles = load_input(&path);
    println!("Finished loading {} particles.", particles.len());
    let closest = particles.iter_mut()
        .map(|p| p.position_at(LONG_TERM))
        .min_by_key(|p| p.mannhattan_distance_to_origin())
        .unwrap();
    println!("Closest particle to the origin: {}", closest.id);
}

fn load_input(path: &str) -> Vec<Particle> {
    let mut file = File::open(path).unwrap();
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();

    buf.lines()
        .filter(|line| !line.chars().all(|c| c.is_whitespace()))
        .enumerate()
        .map(|(i, line)| Particle::from_line(i, line))
        .collect()
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
struct Particle {
    id: usize,
    position: Vector3,
    velocity: Vector3,
    acceleration: Vector3,
}

impl Particle {
    fn from_line(id: usize, line: &str) -> Self {
        let regex = Regex::new(r"(-?\d+)").unwrap();
        let captured_numbers: Vec<_> = regex.captures_iter(line)
            .map(|c| c.get(0).unwrap().as_str())
            .map(|s| s.parse().unwrap())
            .collect();
        let vectors: Vec<_> = captured_numbers.chunks(3).map(Vector3::from).collect();
        assert!(vectors.len() == 3, "invalid number of vectors in line: [{}]", line);
        Self { id, position: vectors[0], velocity: vectors[1], acceleration: vectors[2] }
    }

    fn position_at(self, time: Scalar) -> Self {
        let mut new = self;
        new.position = self.position + self.velocity * time + self.acceleration * time * time;
        new
    }

    fn mannhattan_distance_to_origin(&self) -> Scalar {
        self.position.mannhattan_distance(&Vector3::default())
    }
}