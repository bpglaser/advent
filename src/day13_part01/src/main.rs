use std::collections::HashMap;
use std::env;
use std::fmt;

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn new(x: isize, y: isize) -> Point {
        Point { x: x, y: y }
    }

    fn get_neighbors(&self, designer_number: isize) -> Vec<Point> {
        let neighbors = vec![Point::new(self.x, self.y - 1), // UP
                             Point::new(self.x, self.y + 1), // DOWN
                             Point::new(self.x - 1, self.y), // LEFT
                             Point::new(self.x + 1, self.y), // RIGHT
                             ];

        neighbors.iter()
            .cloned()
            .filter(within_bounds)
            .filter(|p| !is_wall(p, designer_number))
            .collect()
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn main() {
    let args: Vec<isize> =
        env::args().skip(1).map(|s| s.parse().expect("Input must be a number.")).collect();
    if args.len() != 3 {
        panic!("Invalid number of args. (3 required)");
    }
    let designer_number = args[0];
    let start = Point::new(1, 1);
    let goal = Point::new(args[1], args[2]);

    let path = find_path(start, goal, designer_number);

    for y in 0..101 {
        for x in 0..101 {
            let p = Point::new(x, y);
            if is_wall(&p, designer_number) {
                print!("X");
            } else {
                if path.contains(&p) {
                    print!("O");
                } else {
                    print!(".");
                }
            }
        }
        print!("\n");
    }
    for point in path.iter() {
        println!("{}", point);
    }
    println!("Steps required: {}", path.len() - 1);
}

fn within_bounds(p: &Point) -> bool {
    p.x >= 0 && p.y >= 0
}

fn is_wall(p: &Point, designer_number: isize) -> bool {
    let mut total = (p.x * p.x) + (3 * p.x) + (2 * p.x * p.y) + (p.y) + (p.y * p.y);
    total += designer_number;
    bit_count(total) % 2 == 1
}

fn bit_count(mut n: isize) -> isize {
    let mut count = 0;
    while n > 0 {
        if (n & 1) == 1 {
            count += 1;
        }
        n = n >> 1;
    }
    count
}

fn find_path(start: Point, goal: Point, designer_number: isize) -> Vec<Point> {
    let mut frontier: Vec<Point> = vec![start];
    let mut parent_map: HashMap<Point, Option<Point>> = HashMap::new();
    parent_map.insert(start, None);

    while frontier.len() != 0 {
        let point = frontier.remove(0);
        if point == goal {
            return construct_path(goal, parent_map);
        }

        for neighbor in point.get_neighbors(designer_number) {
            if !parent_map.contains_key(&neighbor) {
                parent_map.insert(neighbor, Some(point));
                frontier.push(neighbor);
            }
        }
    }
    panic!("Unable to find path")
}

fn construct_path(goal: Point, mut parent_map: HashMap<Point, Option<Point>>) -> Vec<Point> {
    let mut path = vec![goal];
    let mut working_point = goal;
    while let Some(Some(parent)) = parent_map.remove(&working_point) {
        path.push(parent);
        working_point = parent;
    }
    path.reverse();
    path
}
