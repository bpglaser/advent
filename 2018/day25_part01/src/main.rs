use std::collections::HashMap;
use std::env::args;
use std::fmt;
use std::fs::File;
use std::io::{Read, Write};
use std::str::FromStr;

use petgraph::prelude::*;
use petgraph::algo::connected_components;
use petgraph::dot::Dot;

fn main() {
    let path = args().nth(1).expect("input path");
    let points = load_input(&path);

    let mut graph = Graph::new();

    let mut idxs = HashMap::new();

    for p in &points {
        idxs.insert(p, graph.add_node(p));
    }

    for p in &points {
        for q in &points {
            if p == q {
                continue;
            }

            let dist = p.dist(q);
            if dist <= 3 {
                let pi = idxs[p];
                let qi = idxs[q];
                graph.add_edge(pi, qi, dist);
            }
        }
    }

    println!("{}", connected_components(&graph));

    let mut file = File::create("out.dot").unwrap();
    write!(file, "{}", Dot::new(&graph)).unwrap();
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
    t: i32,
}

impl Point {
    fn dist(&self, other: &Self) -> u32 {
        (self.x - other.x).abs() as u32 +
        (self.y - other.y).abs() as u32 +
        (self.z - other.z).abs() as u32 +
        (self.t - other.t).abs() as u32
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {}, {})", self.x, self.y, self.z, self.t)
    }
}

impl FromStr for Point {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut nums = s.split(',').map(|s| s.parse().unwrap());
        Ok(Self {
            x: nums.next().unwrap(),
            y: nums.next().unwrap(),
            z: nums.next().unwrap(),
            t: nums.next().unwrap(),
        })
    }
}

fn load_input(path: &str) -> Vec<Point> {
    let mut file = File::open(path).unwrap();
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();
    buf.lines().map(|line| line.parse().unwrap()).collect()
}