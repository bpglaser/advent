use std::collections::HashMap;
use std::env::args;
use std::fs::File;
use std::io::Read;

fn main() {
    let path = get_arg();
    let maze = load_maze(&path);
    let zero = maze.iter()
                    .flat_map(|row| row.iter())
                    .find(|node| node.control.map_or_else(|| false, |n| n == 0))
                    .unwrap();
    let two = maze.iter()
                    .flat_map(|row| row.iter())
                    .find(|node| node.control.map_or_else(|| false, |n| n == 2))
                    .unwrap();

    let path = zero.pathfind(*two, &maze);
    println!("path: {:?}", path);
    println!("distance: {}", zero.distance(*two, &maze));

    // let routes = get_routes(&maze);
    //
    // for route in routes {
    //     println!("Path from {:?} to {:?} is distance: {}", route.a, route.b, route.distance);
    // }
}

#[derive(Debug)]
struct Route {
    a: Node,
    b: Node,
    distance: usize,
}

impl Route {
    fn new(a: Node, b: Node, distance: usize) -> Route {
        Route { a: a, b: b, distance: distance }
    }
}

impl PartialEq for Route {
    fn eq(&self, other: &Route) -> bool {
        if self.distance != other.distance {
            return false;
        }
        if self.a == other.a && self.b == other.b {
            return true;
        } else if self.a == other.b && self.b == other.a {
            return true;
        }
        false
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
struct Node {
    x: usize,
    y: usize,
    control: Option<u8>,
    is_passable: bool,
}

impl Node {
    fn new(x: usize, y: usize, c: char) -> Node {
        let mut is_passable = true;
        let mut control = None;
        if c == '#' {
            is_passable = false;
        } else {
            control = c.to_digit(10).map(|n| n as u8);
        }
        Node {
            x: x,
            y: y,
            control: control,
            is_passable: is_passable,
        }
    }

    fn get_neighbors(&self, maze: &Vec<Vec<Node>>) -> Vec<Node> {
        let mut neighbors = vec![];
        if self.y != 0 {
            if let Some(other) = maze.get(self.y - 1).and_then(|row| row.get(self.x)) {
                neighbors.push(*other);
            }
        }
        if let Some(other) = maze.get(self.y + 1).and_then(|row| row.get(self.x)) {
            neighbors.push(*other);
        }
        if self.x != 0 {
            if let Some(other) = maze.get(self.y).and_then(|row| row.get(self.x - 1)) {
                neighbors.push(*other);
            }
        }
        if let Some(other) = maze.get(self.y).and_then(|row| row.get(self.x + 1)) {
            neighbors.push(*other);
        }
        neighbors
    }

    fn distance(&self, goal: Node, maze: &Vec<Vec<Node>>) -> usize {
        self.pathfind(goal, maze).len()
    }

    fn pathfind(&self, goal: Node, maze: &Vec<Vec<Node>>) -> Vec<Node> {
        let mut frontier: Vec<Node> = vec![*self];
        let mut parent_map: HashMap<Node, Option<Node>> = HashMap::new();
        parent_map.insert(*self, None);

        while frontier.len() != 0 {
            let node = frontier.remove(0);

            if node == goal {
                return reconstruct_path(goal, parent_map);
            }

            for neighbor in node.get_neighbors(maze) {
                if neighbor.is_passable && !parent_map.contains_key(&neighbor) {
                    parent_map.insert(neighbor, Some(node));
                    frontier.push(neighbor);
                }
            }
        }
        panic!("Unable to find path")
    }
}

fn reconstruct_path(goal: Node, mut parent_map: HashMap<Node, Option<Node>>) -> Vec<Node> {
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

fn get_routes(maze: &Vec<Vec<Node>>) -> Vec<Route> {
    let mut routes = vec![];

    let controls = get_controls(maze);
    for a in controls.iter() {
        for b in controls.iter() {
            if a != b {
                println!("Finding route between: {:?} and {:?}", a, b);
                let distance = a.distance(*b, maze);
                let route = Route::new(*a, *b, distance);
                if !routes.contains(&route) {
                    println!("Found new route: {:?}", route);
                    routes.push(route);
                }
            }
        }
    }

    routes
}

fn get_controls(maze: &Vec<Vec<Node>>) -> Vec<Node> {
    let mut controls = vec![];
    for row in maze.iter() {
        for node in row.iter() {
            if node.control.is_some() {
                controls.push(*node);
            }
        }
    }
    controls
}

fn load_maze(path: &str) -> Vec<Vec<Node>> {
    let mut file = File::open(path).expect("Error reading file");
    let mut buf = String::new();
    file.read_to_string(&mut buf).expect("Error reading file");

    let mut nodes = vec![];

    for (y, line) in buf.lines().enumerate() {
        let mut row = vec![];
        for (x, c) in line.chars().enumerate() {
            row.push(Node::new(x, y, c));
        }
        nodes.push(row);
    }

    nodes
}

fn get_arg() -> String {
    args().skip(1).next().expect("Invalid args")
}
