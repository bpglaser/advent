use std::collections::HashMap;
use std::env::args;
use std::fs::File;
use std::io::Read;

use Instruction::*;

fn main() {
    let mut powered_lights: HashMap<Point, isize> = HashMap::new();

    let lines = load_lines();
    for (instruction, rect) in lines.iter().map(|s| parse_line(&s)) {
        for point in rect.points() {
            let power_level = powered_lights.entry(point).or_insert(0);
            match instruction {
                On => {
                    *power_level += 1;
                }
                Off => {
                    if *power_level > 0 {
                        *power_level -= 1;
                    }
                }
                Toggle => {
                    *power_level += 2;
                }
            }
        }
    }

    println!("Total power level: {}", powered_lights.values().sum::<isize>());
}

fn parse_line(line: &str) -> (Instruction, Rectangle) {
    let words: Vec<&str> = line.split_whitespace().collect();
    let instruction;
    let a;
    let b;
    if words[0] == "toggle" {
        instruction = Toggle;
        a = Point::parse(words[1]);
        b = Point::parse(words[3]);
    } else {
        if words[1] == "on" {
            instruction = On;
        } else {
            instruction = Off;
        }
        a = Point::parse(words[2]);
        b = Point::parse(words[4]);
    }
    (instruction, Rectangle::new(a, b))
}

#[derive(Eq, PartialEq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Point {
        Point { x: x, y: y }
    }
    fn parse(s: &str) -> Point {
        let split: Vec<usize> = s.split(",").map(|n| n.parse().unwrap()).collect();
        Point::new(split[0], split[1])
    }
}

enum Instruction {
    On,
    Off,
    Toggle,
}

struct Rectangle {
    a: Point,
    b: Point,
}

impl Rectangle {
    fn new(a: Point, b: Point) -> Rectangle {
        Rectangle { a: a, b: b }
    }

    fn points(&self) -> Vec<Point> {
        let mut points = vec![];
        for y in self.a.y..self.b.y + 1 {
            for x in self.a.x..self.b.x + 1 {
                points.push(Point::new(x, y));
            }
        }
        points
    }
}

fn load_lines() -> Vec<String> {
    let path = args().skip(1).next().expect("Invalid args");
    let mut file = File::open(path).expect("Error opening file");
    let mut buf = String::new();
    file.read_to_string(&mut buf).expect("Error reading file");
    buf.lines().map(|s| s.to_owned()).collect()
}
