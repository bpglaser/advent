extern crate crypto;

use crypto::digest::Digest;
use crypto::md5::Md5;

use std::env::args;

fn main() {
    let passcode = get_initial_passcode();
    let start = Point::new_sans_path(0, 0);
    let goal = Point::new_sans_path(3, 3);
    let path = find_path(start, goal, &passcode);
    println!("path: {}", path);
    println!("len: {}", path.len());
}

struct Point {
    x: isize,
    y: isize,
    path: String,
}

impl Point {
    fn new_sans_path(x: isize, y: isize) -> Point {
        Point { x: x, y: y, path: String::new() }
    }

    fn new(x: isize, y: isize, path: String) -> Point {
        Point { x: x, y: y, path: path }
    }

    fn equal_position(&self, other: &Point) -> bool {
        self.x == other.x && self.y == other.y
    }

    fn get_neighbors(&self, passcode: &str) -> Vec<Point> {
        let mut neighbors = vec![];
        let hash = md5_hash(&format!("{}{}", passcode, self.path));
        let hash_chars: Vec<bool> = hash.chars().take(4).map(is_open).collect();
        if hash_chars[0] { // up
            neighbors.push(Point::new(self.x, self.y - 1, self.path.to_owned() + "U"));
        }
        if hash_chars[1] { // down
            neighbors.push(Point::new(self.x, self.y + 1, self.path.to_owned() + "D"));
        }
        if hash_chars[2] { // left
            neighbors.push(Point::new(self.x - 1, self.y, self.path.to_owned() + "L"));
        }
        if hash_chars[3] { // right
            neighbors.push(Point::new(self.x + 1, self.y, self.path.to_owned() + "R"));
        }
        neighbors.retain(within_bounds);
        neighbors
    }
}

fn within_bounds(point: &Point) -> bool {
    point.x >= 0 && point.x <= 3 && point.y >= 0 && point.y <= 3
}

fn is_open(c: char) -> bool {
    match c {
        'b' | 'c' | 'd' | 'e' | 'f' => true,
        _ => false,
    }
}

fn find_path(start: Point, goal: Point, passcode: &str) -> String {
    let mut successful_paths = vec![];
    let mut frontier = vec![start];

    while frontier.len() != 0 {
        let point = frontier.remove(0);
        if point.equal_position(&goal) {
            successful_paths.push(point);
            continue;
        }

        for neighbor in point.get_neighbors(passcode) {
            frontier.push(neighbor);
        }
    }

    successful_paths.into_iter().max_by_key(|p| p.path.len()).expect("Unable to find path").path
}

fn md5_hash(source: &str) -> String {
    let mut digest = Md5::new();
    digest.input_str(source);
    digest.result_str()
}

fn get_initial_passcode() -> String {
    args().skip(1).next().expect("Invalid args")
}
