use std::env::args;
use std::fs::File;
use std::io::Read;
use std::ops::Add;
use std::str::FromStr;

use Direction::*;

fn main() {
    let path = args().nth(1).unwrap();
    let directions = load_input(&path);

    let origin = Coordinate::origin();

    let mut path_taken = vec![origin];
    for direction in directions {
        let next = path_taken.last().unwrap().offset(direction);
        path_taken.push(next);
    }

    let distance = origin.distance_to(path_taken.last().unwrap());
    println!("distance: {}", distance);
}

fn load_input(path: &str) -> Vec<Direction> {
    let mut file = File::open(path).unwrap();
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();
    buf.trim().split(',').map(|s| s.parse().unwrap()).collect()
}

#[derive(Copy, Clone)]
struct Coordinate {
    x: i32,
    y: i32,
    z: i32,
}

impl Coordinate {
    fn origin() -> Self {
        Coordinate { x: 0, y: 0, z: 0 }
    }

    fn offset(&self, direction: Direction) -> Self {
        *self +
            match direction {
                SouthEast => Coordinate { x: 1, y: -1, z: 0 },
                NorthEast => Coordinate { x: 1, y: 0, z: -1 },
                North => Coordinate { x: 0, y: 1, z: -1 },
                NorthWest => Coordinate { x: -1, y: 1, z: 0 },
                SouthWest => Coordinate { x: -1, y: 0, z: 1 },
                South => Coordinate { x: 0, y: -1, z: 1 },
            }
    }

    fn distance_to(&self, other: &Coordinate) -> i32 {
        ((self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()) / 2
    }
}

impl Add for Coordinate {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Coordinate {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

#[derive(Copy, Clone)]
enum Direction {
    SouthEast,
    NorthEast,
    North,
    NorthWest,
    SouthWest,
    South,
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_lowercase().as_str() {
            "se" => SouthEast,
            "ne" => NorthEast,
            "n" => North,
            "nw" => NorthWest,
            "sw" => SouthWest,
            "s" => South,
            _ => panic!("unknown direction: {}", s),
        })
    }
}
