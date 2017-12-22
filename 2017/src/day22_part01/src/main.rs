use std::env::args;
use std::cell::Cell;
use std::fmt;
use std::fs::File;
use std::collections::HashMap;
use std::io::Read;
use std::str::FromStr;

use Direction::*;

fn main() {
    let path = args().nth(1).expect("valid input path");
    let iteration_count = args()
        .nth(2)
        .and_then(|s| s.parse().ok())
        .expect("valid iteration count");
    let mut file = File::open(&path).unwrap();
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();

    let answer = do_puzzle(&buf, iteration_count).unwrap();
    println!("answer: {}", answer);
}

fn do_puzzle(input: &str, iteration_count: u32) -> Result<u32, String> {
    let mut grid = Grid::from_str(input)?;
    for _ in 0..iteration_count {
        grid.tick();
    }
    Ok(grid.infection_count)
}

type Coordinate = (i64, i64);

enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn forward(&self, coordinate: &Coordinate) -> Coordinate {
        match *self {
            North => (coordinate.0, coordinate.1 - 1),
            South => (coordinate.0, coordinate.1 + 1),
            East => (coordinate.0 + 1, coordinate.1),
            West => (coordinate.0 - 1, coordinate.1),
        }
    }

    fn left(&self) -> Self {
        match *self {
            North => West,
            South => East,
            East => North,
            West => South,
        }
    }

    fn right(&self) -> Self {
        match *self {
            North => East,
            South => West,
            East => South,
            West => North,
        }
    }
}

#[derive(Clone, Eq, PartialEq)]
enum Node {
    Clean,
    Infected,
}

struct Carrier {
    position: Coordinate,
    facing: Direction,
}

impl Carrier {
    fn forward(&mut self) {
        self.position = self.facing.forward(&self.position);
    }

    fn left(&mut self) {
        self.facing = self.facing.left();
    }

    fn right(&mut self) {
        self.facing = self.facing.right();
    }
}

struct Grid {
    carrier: Carrier,
    infection_count: u32,
    nodes: HashMap<Coordinate, Node>,
    bounds: Cell<(i64, i64, i64, i64)>,
}

impl Grid {
    fn tick(&mut self) {
        let carrier_position = self.carrier.position;
        if self.get(&carrier_position) == &Node::Infected {
            self.carrier.right();
            *self.get_mut(&carrier_position) = Node::Clean;
        } else {
            self.carrier.left();
            *self.get_mut(&carrier_position) = Node::Infected;
            self.infection_count += 1;
        }
        self.carrier.forward();
        self.update_bounds(&self.carrier.position);
    }

    fn get(&self, coordinate: &Coordinate) -> &Node {
        self.update_bounds(coordinate);
        self.nodes.get(coordinate).unwrap_or(&Node::Clean)
    }

    fn get_mut(&mut self, coordinate: &Coordinate) -> &mut Node {
        self.update_bounds(coordinate);
        if !self.nodes.contains_key(coordinate) {
            self.nodes.insert(coordinate.clone(), Node::Clean);
        }
        self.nodes.get_mut(coordinate).unwrap()
    }

    fn update_bounds(&self, coordinate: &Coordinate) {
        let (mut min_x, mut min_y, mut max_x, mut max_y) = self.bounds.get();
        if min_x > coordinate.0 {
            min_x = coordinate.0;
        }
        if min_y > coordinate.1 {
            min_y = coordinate.1;
        }
        if max_x < coordinate.0 {
            max_x = coordinate.0;
        }
        if max_y < coordinate.1 {
            max_y = coordinate.1;
        }
        self.bounds.set((min_x, min_y, max_x, max_y));
    }
}

impl FromStr for Grid {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut max = (0, 0);
        let mut nodes = HashMap::new();
        for (y, line) in s.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let (x, y) = (x as i64, y as i64);
                let node = match c {
                    '.' => Node::Clean,
                    '#' => Node::Infected,
                    _ => return Err(format!("invalid node char {} at {:?}", c, (x, y))),
                };
                nodes.insert((x, y), node);
                max = (x, y);
            }
        }
        let carrier = Carrier {
            position: (max.0 / 2, max.1 / 2),
            facing: North,
        };
        Ok(Self {
            carrier,
            infection_count: 0,
            nodes,
            bounds: Cell::new((0, 0, max.0, max.1)),
        })
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let padding = 3;
        let (min_x, min_y, max_x, max_y) = self.bounds.get();
        for y in min_y - padding..max_y + padding {
            for x in min_x - padding..max_x + padding {
                if self.carrier.position == (x, y) {
                    match *self.get(&(x, y)) {
                        Node::Clean => write!(f, "[.]")?,
                        Node::Infected => write!(f, "[#]")?,
                    }
                } else {
                    match *self.get(&(x, y)) {
                        Node::Clean => write!(f, " . ")?,
                        Node::Infected => write!(f, " # ")?,
                    }
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[test]
fn test_example() {
    let input = include_str!("../test.txt");
    let mut grid = Grid::from_str(input).unwrap();
    println!("\n{}", grid);
    for _ in 0..7 {
        grid.tick();
        println!("\n{}", grid);
    }
    assert_eq!(5, grid.infection_count);
    for _ in 7..70 {
        grid.tick();
    }
    assert_eq!(41, grid.infection_count);
    for _ in 70..10_000 {
        grid.tick();
    }
    assert_eq!(5587, grid.infection_count);
}
