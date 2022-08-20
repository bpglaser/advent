use std::env::args;
use std::fs::File;
use std::io::Read;
use std::ops::Index;

use pathfinding::directed::dijkstra::dijkstra;

type Position = (i32, i32);

#[derive(Debug)]
struct Graph {
    weights: Vec<i32>,
    width: usize,
    height: usize,
}

impl Graph {
    fn new(s: &str) -> Self {
        let width = s.char_indices().find(|&(_, c)| c == '\n').unwrap().0 - 1;
        let height = s.lines().count();
        let weights = s
            .chars()
            .filter(|c| c.is_numeric())
            .map(|c| c.to_digit(10).unwrap() as i32)
            .collect();
        Graph {
            weights,
            width,
            height,
        }
    }

    fn neighbors(&self, &(x, y): &Position) -> Vec<(Position, i32)> {
        let offsets = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        offsets
            .into_iter()
            .map(|(dx, dy)| (x + dx, y + dy))
            .filter(|&(x, y)| x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32)
            .map(|pos| (pos, self[&pos]))
            .collect()
    }
}

impl Index<&Position> for Graph {
    type Output = i32;

    fn index(&self, &(x, y): &Position) -> &Self::Output {
        let i = x + y * self.width as i32;
        &self.weights[i as usize]
    }
}

fn main() {
    let path = args().nth(1).unwrap();
    let mut f = File::open(path).unwrap();
    let mut buf = String::new();
    f.read_to_string(&mut buf).unwrap();
    let g = Graph::new(&buf);
    let (_, cost) = dijkstra(
        &(0, 0),
        |pos| g.neighbors(pos),
        |pos| pos == &((g.width - 1) as i32, (g.height - 1) as i32),
    )
    .expect("no path found");
    println!("{}", cost);
}
