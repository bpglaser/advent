use regex::Regex;

use vector3::*;

lazy_static! {
    static ref PARTICLE_REGEX: Regex = Regex::new(r"(-?\d+)").unwrap();
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Particle {
    pub id: usize,
    position: Vector3,
    velocity: Vector3,
    acceleration: Vector3,
}

impl Particle {
    pub fn from_line(id: usize, line: &str) -> Self {
        let captured_numbers: Vec<_> = PARTICLE_REGEX.captures_iter(line)
            .map(|c| c.get(0).unwrap().as_str())
            .map(|s| s.parse().unwrap())
            .collect();
        let vectors: Vec<_> = captured_numbers.chunks(3).map(Vector3::from).collect();
        assert!(vectors.len() == 3, "invalid number of vectors in line: [{}]", line);
        Self { id, position: vectors[0], velocity: vectors[1], acceleration: vectors[2] }
    }

    pub fn tick(&mut self) {
        self.velocity += self.acceleration;
        self.position += self.velocity;
    }

    pub fn collides_with(&self, other: &Particle) -> bool {
        self.position == other.position
    }
}