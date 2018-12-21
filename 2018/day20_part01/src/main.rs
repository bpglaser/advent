use std::collections::HashMap;
use std::env::args;
use std::fs::File;
use std::io::Read;

type Coord = (isize, isize);
type Grid = HashMap<Coord, DoorState>;

fn main() {
    let path = args().nth(1).expect("input path");
    let mut file = File::open(&path).unwrap();
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();

    let tokens = lex(&buf[1..buf.len() - 1]);
    println!("{:?}", tokens);

    let nodes = parse(&mut tokens.into_iter());
    println!("{:?}", nodes);

    let mut grid = Grid::new();
    multi_walk(&mut grid, &(0, 0), &nodes);

    render(&grid);
}

fn render(grid: &Grid) {
    let minx = grid.keys().map(|(x, _)| *x).min().unwrap();
    let maxx = grid.keys().map(|(x, _)| *x).max().unwrap();
    let miny = grid.keys().map(|(_, y)| *y).min().unwrap();
    let maxy = grid.keys().map(|(_, y)| *y).max().unwrap();

    for y in miny..=maxy {
        for x in minx..=maxx {
            let doorstate = grid.get(&(x, y)).map(|state| state.0).unwrap_or_default();
            print!("#");
            if doorstate & UP > 0 {
                print!("-");
            } else {
                print!("#");
            }
        }
        println!("#");

        for x in minx..=maxx {
            let doorstate = grid.get(&(x, y)).map(|state| state.0).unwrap_or_default();
            if doorstate & LEFT > 0 {
                print!("|");
            } else {
                print!("#");
            }
            if (x, y) == (0, 0) {
                print!("X");
            } else {
                print!(".");
            }
        }
        println!("#");
    }
}

fn multi_walk(grid: &mut Grid, start: &Coord, nodes: &[Node]) {
    let mut starts = vec![*start];
    for node in nodes {
        let mut next = vec![];
        for start in &starts {
            next.extend(walk(grid, start, node));
        }
        println!("after processing {:?} => {:?}", node, next);
        starts = next;
    }
}

fn walk(grid: &mut Grid, coord: &Coord, node: &Node) -> Vec<Coord> {
    println!("walking at {:?}", coord);
    let mut endpoints = vec![];
    match node {
        Node::Bar => panic!("cannot walk a bar"),
        Node::Raw(s) => {
            let mut coord = *coord;
            for c in s.chars() {
                coord = modify_grid(grid, &coord, c);
            }
            endpoints.push(coord);
        }
        Node::Alternatives(alternatives) => {
            for alt in alternatives {
                endpoints.extend(walk(grid, coord, alt));
            }
        }
        Node::OptionalAlternatives(alternatives) => {
            endpoints.push(*coord);
            for alt in alternatives {
                endpoints.extend(walk(grid, coord, alt));
            }
        }
    }

    endpoints
}

fn modify_grid(grid: &mut Grid, source: &Coord, dir: char) -> Coord {
    let target = move_coord(source, dir);

    match dir {
        'N' => {
            get(grid, source).open_up();
            get(grid, &target).open_down();
        }
        'S' => {
            get(grid, source).open_down();
            get(grid, &target).open_up();
        }
        'W' => {
            get(grid, source).open_left();
            get(grid, &target).open_right();
        }
        'E' => {
            get(grid, source).open_right();
            get(grid, &target).open_left();
        }
        _ => panic!("Invalid move {}", dir),
    }

    target
}

fn get<'a>(grid: &'a mut Grid, coord: &Coord) -> &'a mut DoorState {
    if !grid.contains_key(coord) {
        grid.insert(*coord, DoorState::default());
    }
    grid.get_mut(coord).unwrap()
}

fn move_coord(coord: &Coord, dir: char) -> Coord {
    match dir {
        'N' => (coord.0, coord.1 - 1),
        'S' => (coord.0, coord.1 + 1),
        'W' => (coord.0 - 1, coord.1),
        'E' => (coord.0 + 1, coord.1),
        _ => panic!("Invalid move {}", dir),
    }
}

fn lex(s: &str) -> Vec<String> {
    let mut tokens = vec![];

    let mut buf = String::new();
    for c in s.chars() {
        match c {
            '(' | '|' | ')' => {
                if !buf.is_empty() {
                    tokens.push(buf);
                    buf = String::new();
                }
                tokens.push(format!("{}", c));
            }
            _ => buf.push(c),
        }
    }

    tokens
}

fn parse(tokens: &mut impl Iterator<Item=String>) -> Node {
    let mut children = vec![];

    while let Some(s) = tokens.next() {
        match s.as_str() {
            "(" => {
                let mut alternatives = parse(tokens);
                match alternatives.last() {
                    Some(Kind::Bar) => {
                        alternatives.retain(Kind::is_not_bar);
                        children.push(Kind::OptionalAlternatives(alternatives));
                    }
                    _ => {
                        alternatives.retain(Node::is_not_bar);
                        children.push(Node::Alternatives(alternatives));
                    }
                }
            }
            "|" => children.push(Node::Bar),
            ")" => return children,
            _ => children.push(Node::Raw(s.to_owned())),
        }
    }

    Node { children }
}

#[derive(Debug)]
enum Node {
    Bar,
    Raw(String),
    Sequence(Vec<Node>),
    Alternatives(Vec<Node>),
    OptionalAlternatives(Vec<Node>),
}

impl Node {
    fn is_not_bar(&self) -> bool {
        match self {
            Node::Bar => false,
            _ => true,
        }
    }
}

const UP: u8 = 0b0001;
const DOWN: u8 = 0b0010;
const LEFT: u8 = 0b0100;
const RIGHT: u8 = 0b1000;

#[derive(Debug, Default, Eq, PartialEq)]
struct DoorState(u8);

impl DoorState {
    fn open_up(&mut self) {
        self.0 |= UP;
    }

    fn open_down(&mut self) {
        self.0 |= DOWN;
    }

    fn open_left(&mut self) {
        self.0 |= LEFT;
    }

    fn open_right(&mut self) {
        self.0 |= RIGHT;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn custom() {
        let mut grid = Grid::new();
        // #####
        // #.|.#
        // #.#.#
        // #.#.#
        // #.#.#
        // #####
        walk(&mut grid, &(0, 0), &Node::Raw("NNNESSS".to_owned()));

        let mut given = Grid::new();
        given.insert((0, 0), DoorState(UP));
        given.insert((0, -1), DoorState(UP | DOWN));
        given.insert((0, -2), DoorState(UP | DOWN));
        given.insert((0, -3), DoorState(DOWN | RIGHT));
        given.insert((1, -3), DoorState(DOWN | LEFT));
        given.insert((1, -2), DoorState(UP | DOWN));
        given.insert((1, -1), DoorState(UP | DOWN));
        given.insert((1, 0), DoorState(UP));

        assert_eq!(given, grid);
    }

    #[test]
    fn given_1_0() {
        let mut grid = Grid::new();

        walk(&mut grid, &(0, 0), &Node::Raw("ENWWW".to_owned()));

        let mut expected = Grid::new();
        expected.insert((0, 0), DoorState(RIGHT));
        expected.insert((1, 0), DoorState(UP | LEFT));
        expected.insert((1, -1), DoorState(LEFT | DOWN));
        expected.insert((0, -1), DoorState(LEFT | RIGHT));
        expected.insert((-1, -1), DoorState(LEFT | RIGHT));
        expected.insert((-2, -1), DoorState(RIGHT));

        assert_eq!(expected, grid);
    }

    #[test]
    fn given_1_1() {
        let mut grid = Grid::new();
        let nodes = parse(&mut lex("ENWWW(NEEE|SSE(EE|N))").into_iter());
        multi_walk(&mut grid, &(0, 0), &nodes);

        let mut expected = Grid::new();
        expected.insert((0, 0), DoorState(RIGHT));
        expected.insert((1, 0), DoorState(UP | LEFT));
        expected.insert((1, -1), DoorState(LEFT | DOWN));
        expected.insert((0, -1), DoorState(LEFT | RIGHT));
        expected.insert((-1, -1), DoorState(LEFT | RIGHT));
        expected.insert((-2, -1), DoorState(UP | RIGHT | DOWN));
        expected.insert((-2, -2), DoorState(RIGHT | DOWN));
        expected.insert((-1, -2), DoorState(LEFT | RIGHT));
        expected.insert((0, -2), DoorState(LEFT | RIGHT));
        expected.insert((1, -2), DoorState(LEFT));
        expected.insert((-2, 0), DoorState(UP | DOWN));
        expected.insert((-2, 1), DoorState(UP | RIGHT));
        expected.insert((-1, 1), DoorState(UP | LEFT | RIGHT));
        expected.insert((-1, 0), DoorState(DOWN));
        expected.insert((0, 1), DoorState(LEFT | RIGHT));
        expected.insert((1, 1), DoorState(LEFT));

        for (k, v) in expected {
            if !grid.contains_key(&k) {
                panic!("grid doesn't contain {:?}", k);
            }
            assert_eq!(v, grid[&k], "differs at {:?}", k);
        }
    }
}
