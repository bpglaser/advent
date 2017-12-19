use std::env::args;
use std::fmt;
use std::fs::File;
use std::io::Read;

use Direction::*;
use Token::*;

fn main() {
    let input_path = args().nth(1).expect("valid input path");
    let grid = load_input(&input_path);
    println!("{}", grid);

    let start = grid.iter().find(|&(_, t)| t == &Vertical).unwrap().0;
    println!("start pos: {:?}", start);
    let mut turtle = Turtle::new(start);

    let mut path = vec![];
    while let Some(token) = turtle.step(&grid) {
        path.push(token);
    }

    println!("Path: {}", path.iter().filter(|t| t.is_letter()).map(|t| t.to_string()).collect::<String>());
    println!("Total path length: {}", path.len());
}

fn load_input(path: &str) -> Grid {
    let mut file = File::open(path).unwrap();
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();
    let width = buf.lines().next().unwrap().len();
    let tokens = buf.lines().flat_map(|s| s.chars()).map(Token::from).collect();
    Grid { width, tokens }
}

type Coordinate = (isize, isize);

#[derive(Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn forward(&self, position: Coordinate) -> Coordinate {
        match *self {
            North => (position.0, position.1 - 1),
            South => (position.0, position.1 + 1),
            East => (position.0 + 1, position.1),
            West => (position.0 - 1, position.1),
        }
    }
}

struct Turtle {
    position: Coordinate,
    facing: Direction,
    finished: bool,
}

impl Turtle {
    fn new(position: Coordinate) -> Self {
        Self { position, facing: South, finished: false }
    }

    fn step(&mut self, grid: &Grid) -> Option<Token> {
        if self.finished {
            return None;
        }

        let token = grid.get(self.position);
        let next_position = self.facing.forward(self.position);

        match grid.get(next_position) {
            Some(&Blank) => self.finished = true,
            Some(&Cross) => {
                let northward = North.forward(next_position);
                let southward = South.forward(next_position);
                let eastward = East.forward(next_position);
                let westward = West.forward(next_position);

                let mut found_exit = false;
                
                if northward != self.position {
                    match grid.get(northward) {
                        Some(&Vertical) | Some(&Letter(_)) => {
                            self.facing = North;
                            found_exit = true;
                        }
                        _ => {}
                    }
                }
                if southward != self.position {
                    match grid.get(southward) {
                        Some(&Vertical) | Some(&Letter(_)) => {
                            self.facing = South;
                            found_exit = true;
                        }
                        _ => {}
                    }
                }
                if eastward != self.position {
                    match grid.get(eastward) {
                        Some(&Horizontal) | Some(&Letter(_)) => {
                            self.facing = East;
                            found_exit = true;
                        }
                        _ => {}
                    }
                } 
                if westward != self.position {
                    match grid.get(westward) {
                        Some(&Horizontal) | Some(&Letter(_)) => {
                            self.facing = West;
                            found_exit = true;
                        }
                        _ => {}
                    }
                } 
                if !found_exit {
                    panic!("No exit from cross at {:?}", next_position);
                }
            }
            _ => {}
        }

        self.position = next_position;
        token.cloned()
    }
}

#[derive(Debug)]
struct Grid {
    width: usize,
    tokens: Vec<Token>,
}

impl Grid {
    fn get(&self, coordinate: Coordinate) -> Option<&Token> {
        let i = coordinate.0 + coordinate.1 * self.width as isize;
        if i < 0 {
            None
        } else {
            self.tokens.get(i as usize)
        }
    }

    fn iter(&self) -> Iter {
        Iter { coordinate: (0, 0), grid: self }
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (x, token) in self.tokens.iter().enumerate() {
            write!(f, "{}", token)?;
            if x % self.width == 0 {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

struct Iter<'a> {
    coordinate: Coordinate,
    grid: &'a Grid,
}

impl<'a> Iterator for Iter<'a> {
    type Item = (Coordinate, &'a Token);

    fn next(&mut self) -> Option<Self::Item> {
        let coordinate = self.coordinate;
        if let Some(next) = self.grid.get(self.coordinate) {
            // Successfully got a token; move right
            self.coordinate = (self.coordinate.0 + 1, self.coordinate.1);
            Some((coordinate, next))
        } else {
            // Failed to get a token; try next row
            let coordinate = (0, self.coordinate.1 + 1);
            if let Some(next) = self.grid.get(coordinate) {
                // Sucessfully got a token; move right
                self.coordinate = (1, self.coordinate.1 + 1);
                Some((coordinate, next))
            } else {
                // No token on next row; signal empty
                None
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Token {
    Blank,
    Horizontal,
    Vertical,
    Cross,
    Letter(char),
}

impl Token {
    fn is_letter(&self) -> bool {
        match self {
            &Letter(_) => true,
            _ => false,
        }
    }
}

impl From<char> for Token {
    fn from(c: char) -> Self {
        match c {
            ' ' => Blank,
            '-' => Horizontal,
            '|' => Vertical,
            '+' => Cross,
            _ => Letter(c),
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}",
            match *self {
                Blank => ' ',
                Horizontal => '-',
                Vertical => '|',
                Cross => '+',
                Letter(c) => c,
            }
        )
    }
}