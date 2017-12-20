#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::env::args;
use std::fs::File;
use std::io::Read;

mod particle;
mod vector3;

use particle::Particle;

const LONG_TERM: i64 = 1000;

fn main() {
    let path = args().nth(1).expect("valid input path");
    println!("Loading input...");
    let mut particles = load_input(&path);
    println!("Finished loading {} particles.", particles.len());

    for _ in 0..LONG_TERM {
        particles.iter_mut().for_each(Particle::tick);

        let mut to_remove = vec![];
        for (i, a) in particles.iter().enumerate() {
            for (j, b) in particles.iter().enumerate() {
                if i != j && a.collides_with(b) {
                    to_remove.push(i);
                    to_remove.push(j);
                }
            }
        }

        particles = particles.into_iter()
            .enumerate()
            .filter(|&(i, _)| !to_remove.contains(&i))
            .map(|(_, p)| p)
            .collect();
    }
    
    println!("{} remaining particles", particles.len());
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
