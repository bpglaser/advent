use std::collections::HashMap;
use std::env::args;
use std::fs::File;
use std::io::Read;

fn main() {
    let path = get_arg();
    let maze = load_maze(&path);

    let mut control_nodes = find_nodes(&maze, |node| node.control.is_some());
    control_nodes.sort_by_key(|node| node.control.unwrap());
    let start = control_nodes[0];

    let routes = get_routes(&maze);
    println!("All routes:");
    for route in routes.iter() {
        println!("\t{:?}", route);
    }
    println!("Number of routes: {}", routes.len());

    let optimal_route = find_optimal_route(start, &control_nodes, &routes);
    let optimal_distance = sum_route(&optimal_route);

    println!("Optimal route:");
    for route in optimal_route {
        println!("\t{:?}", route);
    }
    println!("Optimal distance: {}", optimal_distance);
}

fn find_optimal_route<'a>(start: Node, nodes: &[Node], routes: &'a [Route]) -> Vec<&'a Route> {
    let mut optimal_route = None;

    let permutations = get_node_permutations(nodes);
    for mut permutation in permutations {
        let mut possible_route = vec![];
        let mut previous_node = permutation.remove(0);

        if previous_node != start {
            continue;
        }

        for next_node in permutation.iter() {
            let route = routes.iter()
                                .find(|route| route.is_between(previous_node, *next_node))
                                .expect(&format!("Failed to find route between {:?} and {:?}", previous_node, next_node));
            possible_route.push(route);
            previous_node = *next_node;
        }

        if optimal_route.is_none() {
            optimal_route = Some(possible_route);
        } else {
            if sum_route(&possible_route) < sum_route(optimal_route.as_ref().unwrap()) {
                optimal_route = Some(possible_route);
            }
        }
    }

    optimal_route.expect("Failed to find optimal route")
}

fn sum_route(routes: &[&Route]) -> usize {
    routes.iter().map(|route| route.distance).sum()
}

fn get_node_permutations(nodes: &[Node]) -> Vec<Vec<Node>> {
    let mut permutation = clone_nodes(nodes);

    let mut c = vec![0; nodes.len()];

    let mut all_permutations = vec![clone_nodes(nodes)];

    let mut i = 0;
    while i < nodes.len() {
        if c[i] < i {
            if i % 2 == 0 {
                let temp = permutation[0];
                permutation[0] = permutation[i];
                permutation[i] = temp;
            } else {
                let temp = permutation[c[i]];
                permutation[c[i]] = permutation[i];
                permutation[i] = temp;
            }
            all_permutations.push(clone_nodes(&permutation));
            c[i] += 1;
            i = 0;
        } else {
            c[i] = 0;
            i += 1;
        }
    }

    all_permutations
}

fn clone_nodes(nodes: &[Node]) -> Vec<Node> {
    nodes.iter().cloned().collect()
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

    fn is_between(&self, a: Node, b: Node) -> bool {
        (self.a == a && self.b == b) || (self.a == b && self.b == a)
    }
}

impl PartialEq for Route {
    fn eq(&self, other: &Route) -> bool {
        if self.distance != other.distance {
            return false;
        }
        self.is_between(other.a, other.b)
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

    fn get_neighbors(&self, maze: &[Vec<Node>]) -> Vec<Node> {
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

    fn distance(&self, goal: Node, maze: &[Vec<Node>]) -> usize {
        self.pathfind(goal, maze).len()
    }

    fn pathfind(&self, goal: Node, maze: &[Vec<Node>]) -> Vec<Node> {
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

fn get_routes(maze: &[Vec<Node>]) -> Vec<Route> {
    let mut routes = vec![];

    let controls = get_controls(maze);
    for a in controls.iter() {
        for b in controls.iter() {
            if a != b {
                let distance = a.distance(*b, maze);
                let route = Route::new(*a, *b, distance);
                if !routes.contains(&route) {
                    routes.push(route);
                }
            }
        }
    }

    routes
}

fn get_controls(maze: &[Vec<Node>]) -> Vec<Node> {
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

fn find_nodes<P>(nodes: &[Vec<Node>], predicate: P) -> Vec<Node> where P: Fn(&Node) -> bool {
    nodes.iter().flat_map(|row| row.iter()).filter(|node| predicate(node)).map(|node| *node).collect()
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
