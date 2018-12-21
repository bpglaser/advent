use std::cmp;
use std::collections::{HashMap, HashSet};
use std::env::args;
use std::fs::File;
use std::io::Read;

fn main() {
    let path = args().nth(1).expect("input path");
    let mut file = File::open(&path).unwrap();
    let mut buf = String::new();

    file.read_to_string(&mut buf).unwrap();
    let buf: String = buf.chars().skip(1).take_while(|c| c != &'$').collect();

    println!("{}", solve(&buf));
}

fn solve(s: &str) -> usize {
    let mut x = 0;
    let mut y = 0;
    let mut distances: HashMap<(isize, isize), usize> = HashMap::new();
    let mut graph = HashMap::new();
    let mut positions = vec![];

    for c in s.chars() {
        let prev = (x, y);
        match c {
            '(' => {
                positions.push((x, y));
            }
            ')' => {
                let pos = positions.pop().unwrap();
                x = pos.0;
                y = pos.1;
            }
            '|' => {
                let pos = positions.last().unwrap();
                x = pos.0;
                y = pos.1;
            }
            _ => {
                match c {
                    'N' => y -= 1,
                    'S' => y += 1,
                    'W' => x -= 1,
                    'E' => x += 1,
                    _ => unreachable!(),
                }

                graph
                    .entry((x, y))
                    .or_insert_with(HashSet::new)
                    .insert(prev);

                let dist = *distances.get(&(x, y)).unwrap_or(&0);
                if dist != 0 {
                    let dist = cmp::min(dist, *distances.get(&prev).unwrap_or(&0) + 1);
                    distances.insert((x, y), dist);
                } else {
                    distances.insert((x, y), *distances.get(&prev).unwrap_or(&0) + 1);
                }
            }
        }
    }

    *distances.values().max().unwrap()
}
