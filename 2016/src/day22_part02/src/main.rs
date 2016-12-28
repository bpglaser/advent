use std::collections::HashMap;
use std::env::args;
use std::fs::File;
use std::io::Read;

fn main() {
    let path = get_path();
    let nodes = load_nodes(&path);
    let width = nodes[0].len();

    let mut empty_point = None;
    for (y, row) in nodes.iter().enumerate() {
        for (x, node) in row.iter().enumerate() {
            if node.used == 0 {
                print!("_");
                empty_point = Some(Point::new(x, y));
            } else if node.is_wall() {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    let empty_point = empty_point.expect("Unable to find empty node");
    let goal_point = Point::new(width - 2, 0);

    let dist_to_goal = empty_point.pathfind_to(goal_point, &nodes).len();
    let answer = 1 + dist_to_goal + (width - 2) * 5;
    println!("Answer : {}", answer);
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Point {
        Point { x: x, y: y }
    }

    fn get_neighbors(&self, maze: &[Vec<Node>]) -> Vec<Point> {
        let mut neighbors = vec![];
        if self.y != 0 {
            if let Some(other) = maze.get(self.y - 1).and_then(|row| row.get(self.x)) {
                if !other.is_wall() {
                    neighbors.push(Point::new(self.x, self.y - 1));
                }
            }
        }
        if let Some(other) = maze.get(self.y + 1).and_then(|row| row.get(self.x)) {
            if !other.is_wall() {
                neighbors.push(Point::new(self.x, self.y + 1));
            }
        }
        if self.x != 0 {
            if let Some(other) = maze.get(self.y).and_then(|row| row.get(self.x - 1)) {
                if !other.is_wall() {
                    neighbors.push(Point::new(self.x - 1, self.y));
                }
            }
        }
        if let Some(other) = maze.get(self.y).and_then(|row| row.get(self.x + 1)) {
            if !other.is_wall() {
                neighbors.push(Point::new(self.x + 1, self.y));
            }
        }
        neighbors
    }

    fn pathfind_to(&self, goal: Point, maze: &[Vec<Node>]) -> Vec<Point> {
        let mut frontier: Vec<Point> = vec![*self];
        let mut parent_map: HashMap<Point, Option<Point>> = HashMap::new();
        parent_map.insert(*self, None);

        while frontier.len() != 0 {
            let point = frontier.remove(0);

            if point == goal {
                return reconstruct_path(point, parent_map);
            }

            for neighbor in point.get_neighbors(maze) {
                if !parent_map.contains_key(&neighbor) {
                    parent_map.insert(neighbor, Some(point));
                    frontier.push(neighbor);
                }
            }
        }
        panic!("Unable to find path")
    }
}

#[derive(Clone, Copy, Debug)]
struct Node {
    size: usize,
    used: usize,
    avail: usize,
}

impl Node {
    fn from_line(line: &str) -> (usize, usize, Node) {
        let words: Vec<&str> = line.split_whitespace().map(|s| s.trim()).collect();
        let filesystem = words[0];
        let (x, y) = parse_coords(&filesystem);
        (x, y, Node {
            size: parse_volume(words[1]),
            used: parse_volume(words[2]),
            avail: parse_volume(words[3]),
        })
    }

    fn is_wall(&self) -> bool {
        self.used > 100
    }
}

fn reconstruct_path(goal: Point, mut parent_map: HashMap<Point, Option<Point>>) -> Vec<Point> {
    let mut path = vec![goal];

    let mut working_node = goal;
    while let Some(Some(parent)) = parent_map.remove(&working_node) {
        path.push(parent);
        working_node = parent;
    }

    path.pop();
    path.reverse();
    path
}

fn parse_coords(s: &str) -> (usize, usize) {
    let mut x_encountered = false;
    let mut x = String::new();
    let mut y_encountered = false;
    let mut y = String::new();
    for c in s.chars() {
        if c == 'x' {
            x_encountered = true;
        } else if c == 'y' {
            y_encountered = true
        } else if c.is_digit(10) {
            if y_encountered {
                y.push(c);
            } else if x_encountered {
                x.push(c);
            } else {
                panic!();
            }
        }
    }
    (x.parse().expect("Error parsing coords"), y.parse().expect("Error parsing coords"))
}

fn parse_volume(s: &str) -> usize {
    s.chars().take_while(|c| c.is_digit(10)).collect::<String>().parse().expect(&format!("Error parsing: {}", s))
}

fn load_nodes(path: &str) -> Vec<Vec<Node>> {
    let mut file = File::open(path).expect(&format!("Error opening: {}", path));
    let mut buf = String::new();
    file.read_to_string(&mut buf).expect(&format!("Error reading from: {}", path));

    let mut all_nodes: Vec<_> = buf.lines().skip(2).map(Node::from_line).collect();
    all_nodes.sort_by_key(|&(x, ..)| x); // Pre-sorting the list ensures nodes are appended to each row in order

    let mut array: Vec<Vec<Node>> = vec![];

    for &mut (_, y, node) in all_nodes.iter_mut() {
        loop { // Loop ensures that rows are initialized regardless of y pos input
            if let Some(row) = array.get_mut(y) {
                row.push(node);
                break;
            }
            array.push(vec![]);
        }
    }

    array
}

fn get_path() -> String {
    args().skip(1).next().expect("Invalid input")
}
