use std::cmp;
use std::env::args;
use std::fs::File;
use std::io::Read;
use std::io::stdout;
use std::str::FromStr;

use lazy_static::lazy_static;
use regex::Regex;

use crate::CoordRange::*;

// TOO LOW 1308

type Grid = Vec<Vec<char>>;

lazy_static! {
    static ref PARSE_PATTERN: Regex = Regex::new(r".*?(\d+).*?(\d+).*?(\d+)").unwrap();
}

const SPRING: (usize, usize) = (500, 0);

fn main() {
    let mut grid;
    let mut spring_x = SPRING.0;
    let mut spring_y = SPRING.1;
    if let Some(debug_path) = args().nth(3) {
        let debug = load_debug(&debug_path);
        grid = debug.0;
        spring_x = debug.1;
        spring_y = debug.2;
    } else {
        let path = args().nth(1).expect("input path");

        let coords = read_input(&path);
        let ((minx, maxx), (miny, maxy)) = find_bounds(&coords);

        grid = vec![vec!['.'; maxx + 2]; maxy + 1];
        for coord in &coords {
            for (x, y) in coord.iter() {
                grid[y][x] = '#';
            }
        }
        grid[spring_y][spring_x] = '+';

        render(&grid, &mut stdout()); // todo remove
    }

    fill(&mut grid, spring_x, spring_y + 1);

    if let Some(out) = args().nth(2) {
        let mut file = File::create(out).unwrap();
        render(&grid, &mut file);
    }

    let mut count = 0;
    for row in &grid {
        for c in row {
            match c {
                '|' | '~' => count += 1,
                _ => {}
            }
        }
    }
    println!("ans: {}", count);
}

fn fill(grid: &mut Grid, start_x: usize, start_y: usize) {
    let mut stack = vec![(start_x, start_y)];
    while let Some((start_x, start_y)) = stack.pop() {
        println!("filling from {:?}", (start_x, start_y));

        if start_y == grid.len() - 1 {
            grid[start_y][start_x] = '|';
            return;
        }

        if grid[start_y][start_x] != '.' {
            continue;
        }

        let mut furthest_y = start_y;
        for y in start_y..grid.len() {
            if y == grid.len() - 1 {
                grid[y][start_x] = '|';
                break;
            }

            match grid[y][start_x] {
                '#' | '~' => {
                    break;
                }
                '.' | '|' => {
                    furthest_y = y;
                    grid[y][start_x] = '|';
                }
                _ => panic!("invalid state"),
            }
        }

        for y in (0..=furthest_y).rev() {
            // if grid[y][start_x] == '~' {
            //     continue;
            // }
            if grid[y + 1][start_x] == '~' || grid[y + 1][start_x] == '#' {
                let (left, right) = spread(grid, start_x, y);
                if let Some(left) = left {
                    stack.push(left);
                }
                if let Some(right) = right {
                    stack.push(right);
                }
            }
        }
        println!("after fill");
        render(&grid, &mut stdout());
    }
}

fn spread(grid: &mut Grid, start_x: usize, start_y: usize) -> (Option<(usize, usize)>, Option<(usize, usize)>) {
    println!("spreading at {:?}", (start_x, start_y));
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
        println!("filled up");
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
            return (Some((far_left - 1, start_y)), Some((far_right + 1, start_y)));
        }
    }
    return (None, None)
}

fn find_bounds(coords: &[CoordRange]) -> ((usize, usize), (usize, usize)) {
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

fn load_debug(path: &str) -> (Grid, usize, usize) {
    let mut file = File::open(path).unwrap();
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();
    let grid: Vec<Vec<char>> = buf.lines().map(|line| line.chars().collect()).collect();

    for (y, row) in grid.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if c == &'+' {
                return (grid, x, y);
            }
        }
    }
    panic!("no cross found in debug");
}

fn render<T: std::io::Write>(grid: &Grid, w: &mut T) {
    for row in grid{
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
    Top,
    SpreadLeft,
    SpreadRight,
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
