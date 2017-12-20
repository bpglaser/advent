use std::env::args;
use std::fs::File;
use std::io::Read;

mod vector3;
use vector3::*;

const LONG_TERM: i64 = 1_000_000_000;

fn main() {
    let path = args().nth(1).expect("valid input path");
    let mut particles = load_input(&path);
    let closest = particles.iter_mut()
        .map(|p| p.position_at(LONG_TERM))
        .min_by_key(|p| p.mannhattan_distance_to_origin())
        .unwrap();
    println!("Closest: {}", closest.id);
}

fn load_input(path: &str) -> Vec<Particle> {
    let mut file = File::open(path).unwrap();
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();

    buf.lines()
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
        unimplemented!()
    }

    fn position_at(self, time: Scalar) -> Self {
        let mut new = self.clone();
        new.position = self.position + self.velocity * time + self.acceleration * time * time;
        new
    }

    fn mannhattan_distance_to_origin(&self) -> Scalar {
        self.position.mannhattan_distance(&Vector3::default())
    }
}