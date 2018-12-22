use std::collections::HashMap;
use std::env::args;
use std::fs::File;
use std::io::Read;

fn main() {
    let path = args().nth(1).unwrap();
    let (depth, coord) = read_input(&path);

    let mut state = State::new(depth, coord);

    let mut ans = 0;
    for y in 0..=coord.1 {
        for x in 0..=coord.0 {
            ans += state.calc_type(x, y);
        }
    }

    println!("ans {}", ans);
}

type Cache = HashMap<(usize, usize), usize>;

struct State {
    depth: usize,
    index_cache: Cache,
    erosion_cache: Cache,
    type_cache: Cache,
}

impl State {
    fn new(depth: usize, coord: (usize, usize)) -> Self {
        let mut state = State {
            depth,
            index_cache: Cache::new(),
            erosion_cache: Cache::new(),
            type_cache: Cache::new(),
        };
        state.index_cache.insert(coord, 0);
        state
    }

    fn calc_geological_index(&mut self, x: usize, y: usize) -> usize {
        if !self.index_cache.contains_key(&(x, y)) {
            let index = if x == 0 {
                y * 48271
            } else if y == 0 {
                x * 16807
            } else {
                self.calc_erosion_level(x - 1, y) * self.calc_erosion_level(x, y - 1)
            };
            self.index_cache.insert((x, y), index);
        }
        self.index_cache[&(x, y)]
    }

    fn calc_erosion_level(&mut self, x: usize, y: usize) -> usize {
        if !self.erosion_cache.contains_key(&(x, y)) {
            let erosion_level = (self.calc_geological_index(x, y) + self.depth) % 20183;
            self.erosion_cache.insert((x, y), erosion_level);
        }
        self.erosion_cache[&(x, y)]
    }

    fn calc_type(&mut self, x: usize, y: usize) -> usize {
        if !self.type_cache.contains_key(&(x, y)) {
            let t = self.calc_erosion_level(x, y) % 3;
            self.type_cache.insert((x, y), t);
        }
        self.type_cache[&(x, y)]
    }
}

fn read_input(path: &str) -> (usize, (usize, usize)) {
    let mut file = File::open(path).unwrap();
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();
    let mut lines = buf.lines();

    let depth = lines
        .next()
        .unwrap()
        .split_whitespace()
        .nth(1)
        .unwrap()
        .parse()
        .unwrap();

    let mut pair = lines
        .next()
        .unwrap()
        .split_whitespace()
        .nth(1)
        .unwrap()
        .split(',');

    let x = pair.next().unwrap().parse().unwrap();
    let y = pair.next().unwrap().parse().unwrap();

    (depth, (x, y))
}
