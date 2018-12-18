use std::cmp;
use std::env::args;
use std::fs::File;
use std::io::Read;
use std::str::FromStr;

use lazy_static::lazy_static;
use regex::Regex;

use crate::CoordRange::*;
use crate::FlowDirection::*;

type Coord = (usize, usize);
type FlowStack = Vec<(usize, usize, FlowDirection)>;
type Grid = Vec<Vec<char>>;

lazy_static! {
    static ref PARSE_PATTERN: Regex = Regex::new(r".*?(\d+).*?(\d+).*?(\d+)").unwrap();
}

const SPRING: Coord = (500, 0);

fn main() {
    let mut grid;

    let path = args().nth(1).expect("input path");

    let coords = read_input(&path);
    let ((minx, maxx), (miny, maxy)) = find_bounds(&coords);

    grid = vec![vec!['.'; maxx + 2]; maxy + 1];
    for coord in &coords {
        for (x, y) in coord.iter() {
            grid[y][x] = '#';
        }
    }
    grid[SPRING.1][SPRING.0] = '+';

    fill(&mut grid, SPRING.0, SPRING.1 + 1);

    if let Some(out) = args().nth(2) {
        let mut file = File::create(out).unwrap();
        render(&grid, &mut file);
    }

    let mut count = 0;
    for y in miny..=maxy {
        for c in &grid[y] {
            match c {
                '~' => count += 1,
                _ => {}
            }
        }
    }
    println!("ans: {}", count);
}

fn fill(grid: &mut Grid, start_x: usize, start_y: usize) {
    let mut stack = vec![(start_x, start_y, Down)];
    while let Some((start_x, start_y, dir)) = stack.pop() {
        match dir {
            Down => flow_down(&mut stack, grid, start_x, start_y),
            Up => flow_up(&mut stack, grid, start_x, start_y),
        }
    }
}

fn flow_down(stack: &mut FlowStack, grid: &mut Grid, start_x: usize, start_y: usize) {
    for y in start_y..grid.len() {
        match grid[y][start_x] {
            '~' | '#' => break,
            '.' | '|' => {
                grid[y][start_x] = '|';
                stack.push((start_x, y, Up));
            }
            _ => unreachable!(),
        }
    }
}

fn flow_up(stack: &mut FlowStack, grid: &mut Grid, start_x: usize, start_y: usize) {
    if start_y == grid.len() - 1 {
        return;
    }
    match grid[start_y + 1][start_x] {
        '~' | '#' => match spread(grid, start_x, start_y) {
            (Some(l), Some(r)) => {
                stack.push((l.0, l.1, Down));
                stack.push((r.0, r.1, Down));
            }
            (Some(l), None) => stack.push((l.0, l.1, Down)),
            (None, Some(r)) => stack.push((r.0, r.1, Down)),
            (None, None) => {}
        },
        _ => {}
    }
}

fn spread(grid: &mut Grid, start_x: usize, start_y: usize) -> (Option<Coord>, Option<Coord>) {
    if start_y == grid.len() - 1 || grid[start_y][start_x] == '#' {
        return (None, None);
    }

    let mut left_wall = false;
    let mut far_left = start_x;
    for x in (1..=start_x).rev() {
        if grid[start_y][x] == '#' {
            left_wall = true;
            break;
        }

        match grid[start_y + 1][x] {
            '#' | '~' => far_left = x,
            '.' | '|' => break,
            _ => panic!("invalid state"),
        }
    }

    let mut right_wall = false;
    let mut far_right = start_x;
    for x in start_x..grid[0].len() - 1 {
        if grid[start_y][x] == '#' {
            right_wall = true;
            break;
        }

        match grid[start_y + 1][x] {
            '#' | '~' => far_right = x,
            '.' | '|' => break,
            _ => panic!("invalid state"),
        }
    }

    if left_wall && right_wall {
        for x in far_left..=far_right {
            grid[start_y][x] = '~';
        }
        return (None, None);
    } else {
        for x in far_left..=far_right {
            grid[start_y][x] = '|';
        }
        if !left_wall && right_wall {
            return (Some((far_left - 1, start_y)), None);
        }
        if left_wall && !right_wall {
            return (None, Some((far_right + 1, start_y)));
        }
        if !left_wall && !right_wall {
            return (
                Some((far_left - 1, start_y)),
                Some((far_right + 1, start_y)),
            );
        }
    }
    return (None, None);
}

fn find_bounds(coords: &[CoordRange]) -> (Coord, Coord) {
    let mut minx = usize::max_value();
    let mut maxx = usize::min_value();
    let mut miny = usize::max_value();
    let mut maxy = usize::min_value();

    for coord in coords {
        match coord {
            XMajor { x_start, x_end, y } => {
                minx = cmp::min(minx, *x_start);
                maxx = cmp::max(maxx, *x_end);
                miny = cmp::min(miny, *y);
                maxy = cmp::max(maxy, *y);
            }
            YMajor { x, y_start, y_end } => {
                minx = cmp::min(minx, *x);
                maxx = cmp::max(maxx, *x);
                miny = cmp::min(miny, *y_start);
                maxy = cmp::max(maxy, *y_end);
            }
        }
    }

    ((minx, maxx), (miny, maxy))
}

fn render<T: std::io::Write>(grid: &Grid, w: &mut T) {
    for row in grid {
        for c in row {
            write!(w, "{}", c).unwrap();
        }
        writeln!(w).unwrap();
    }
}

fn read_input(path: &str) -> Vec<CoordRange> {
    let mut buf = String::new();
    let mut file = File::open(path).unwrap();
    file.read_to_string(&mut buf).unwrap();
    buf.lines().map(|line| line.parse().unwrap()).collect()
}

enum FlowDirection {
    Down,
    Up,
}

#[derive(Debug)]
enum CoordRange {
    XMajor {
        x_start: usize,
        x_end: usize,
        y: usize,
    },
    YMajor {
        x: usize,
        y_start: usize,
        y_end: usize,
    },
}

impl CoordRange {
    fn iter(&self) -> Box<Iterator<Item = (usize, usize)>> {
        match *self {
            XMajor { x_start, x_end, y } => Box::new((x_start..=x_end).map(move |x| (x, y))),
            YMajor { x, y_start, y_end } => Box::new((y_start..=y_end).map(move |y| (x, y))),
        }
    }
}

impl FromStr for CoordRange {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.chars().next().unwrap() {
            'x' => {
                let caps: Vec<_> = PARSE_PATTERN
                    .captures(s)
                    .unwrap()
                    .iter()
                    .skip(1)
                    .map(|s| s.unwrap().as_str().parse().unwrap())
                    .collect();
                Ok(CoordRange::YMajor {
                    x: caps[0],
                    y_start: caps[1],
                    y_end: caps[2],
                })
            }
            'y' => {
                let caps: Vec<_> = PARSE_PATTERN
                    .captures(s)
                    .unwrap()
                    .iter()
                    .skip(1)
                    .map(|s| s.unwrap().as_str().parse().unwrap())
                    .collect();
                Ok(CoordRange::XMajor {
                    y: caps[0],
                    x_start: caps[1],
                    x_end: caps[2],
                })
            }
            _ => panic!("invalid line: {}", s),
        }
    }
}
