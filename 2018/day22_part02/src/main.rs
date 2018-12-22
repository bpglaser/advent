use std::collections::{HashMap, HashSet};
use std::env::args;
use std::fs::File;
use std::io::Read;

use crate::Equipment::*;
use crate::Terrain::*;

const INF: usize = usize::max_value() / 2;

fn main() {
    let path = args().nth(1).unwrap();
    let (depth, coord) = read_input(&path);

    let mut state = State::new(depth, coord);

    let path = pathfind(
        &mut state,
        Position::new((0, 0), Torch),
        Position::new(coord, Torch),
    );

    let mut ans = 0;
    for i in 0..path.len() - 1 {
        ans += dist_between(&mut state, &path[i], &path[i + 1]);
    }
    println!("ans: {}", ans);
}

// A* pathfing algo
fn pathfind(state: &mut State, start: Position, goal: Position) -> Vec<Position> {
    let mut closed_set = HashSet::new();
    let mut open_set = HashSet::new();
    open_set.insert(start);
    let mut came_from = HashMap::new();

    let mut g_score: HashMap<Position, usize> = HashMap::new();
    g_score.insert(start, 0);

    let mut f_score: HashMap<Position, usize> = HashMap::new();
    f_score.insert(start, heuristic_cost_estimate(&start, &goal));

    while !open_set.is_empty() {
        let current = *open_set.iter().min_by_key(|coord| f_score[coord]).unwrap();

        if current == goal {
            return reconstruct_path(&came_from, current);
        }

        open_set.remove(&current);
        closed_set.insert(current);

        for neighbor in get_neighbors(state, &current) {
            if closed_set.contains(&neighbor) {
                continue;
            }

            let tentative_g_score =
                g_score.get(&current).unwrap_or(&INF) + dist_between(state, &current, &neighbor);

            if !open_set.contains(&neighbor) {
                open_set.insert(neighbor);
            } else if tentative_g_score >= *g_score.get(&neighbor).unwrap_or(&INF) {
                continue;
            }

            came_from.insert(neighbor, current);
            g_score.insert(neighbor, tentative_g_score);
            f_score.insert(
                neighbor,
                g_score[&neighbor] + heuristic_cost_estimate(&neighbor, &goal),
            );
        }
    }

    panic!("No path");
}

fn dist_between(state: &mut State, start: &Position, goal: &Position) -> usize {
    if start.x == goal.x && start.y == goal.y {
        if start.equipped == goal.equipped {
            return 0;
        }

        let terrain = state.calc_type(start.x, start.y);
        if start.equipped.valid_swap(terrain) != goal.equipped {
            panic!("Invalid state. Cannot move from {:?} to {:?}", start, goal);
        }

        return 7;
    }

    if heuristic_cost_estimate(start, goal) != 1 {
        panic!("Invalid state. Cannot move from {:?} to {:?}", start, goal);
    }

    1
}

fn get_neighbors(state: &mut State, node: &Position) -> Vec<Position> {
    let mut neighbors = Vec::with_capacity(5);
    neighbors.push(node.swap_equipped(state));
    if let Some(up) = node.up(state) {
        neighbors.push(up);
    }
    if let Some(down) = node.down(state) {
        neighbors.push(down);
    }
    if let Some(left) = node.left(state) {
        neighbors.push(left);
    }
    if let Some(right) = node.right(state) {
        neighbors.push(right);
    }
    neighbors
}

fn reconstruct_path(
    came_from: &HashMap<Position, Position>,
    mut current: Position,
) -> Vec<Position> {
    let mut path = vec![current];

    while came_from.contains_key(&current) {
        current = came_from[&current];
        path.push(current);
    }

    path.into_iter().rev().collect()
}

// Manhattan distance
fn heuristic_cost_estimate(start: &Position, goal: &Position) -> usize {
    let dx = start
        .x
        .checked_sub(goal.x)
        .unwrap_or_else(|| goal.x - start.x);
    let dy = start
        .y
        .checked_sub(goal.y)
        .unwrap_or_else(|| goal.y - start.y);
    dx + dy
}

#[derive(Clone, Copy, Debug)]
enum Terrain {
    Rocky,
    Wet,
    Narrow,
}

impl Terrain {
    fn is_valid(self, equipment: Equipment) -> bool {
        match (self, equipment) {
            (Rocky, ClimbingGear) => true,
            (Rocky, Torch) => true,
            (Wet, ClimbingGear) => true,
            (Wet, Neither) => true,
            (Narrow, Torch) => true,
            (Narrow, Neither) => true,
            _ => false,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Equipment {
    Torch,
    ClimbingGear,
    Neither,
}

impl Equipment {
    fn valid_swap(self, terrain: Terrain) -> Self {
        match (self, terrain) {
            (Torch, Rocky) => ClimbingGear,
            (Torch, Narrow) => Neither,
            (ClimbingGear, Rocky) => Torch,
            (ClimbingGear, Wet) => Neither,
            (Neither, Wet) => ClimbingGear,
            (Neither, Narrow) => Torch,
            _ => panic!("Invalid state: in {:?} with {:?} equipped", terrain, self),
        }
    }
}

type Coord = (usize, usize);
type Cache<T> = HashMap<Coord, T>;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Position {
    x: usize,
    y: usize,
    equipped: Equipment,
}

impl Position {
    fn new(coord: Coord, equipped: Equipment) -> Self {
        Self {
            x: coord.0,
            y: coord.1,
            equipped,
        }
    }

    fn swap_equipped(&self, state: &mut State) -> Self {
        let terrain = state.calc_type(self.x, self.y);
        Self {
            x: self.x,
            y: self.y,
            equipped: self.equipped.valid_swap(terrain),
        }
    }

    fn up(&self, state: &mut State) -> Option<Self> {
        if self.y == 0 {
            None
        } else {
            let candidate = Position::new((self.x, self.y - 1), self.equipped);
            if state.validate_position(&candidate) {
                Some(candidate)
            } else {
                None
            }
        }
    }

    fn down(&self, state: &mut State) -> Option<Self> {
        let candidate = Position::new((self.x, self.y + 1), self.equipped);
        if state.validate_position(&candidate) {
            Some(candidate)
        } else {
            None
        }
    }

    fn left(&self, state: &mut State) -> Option<Self> {
        if self.x == 0 {
            None
        } else {
            let candidate = Position::new((self.x - 1, self.y), self.equipped);
            if state.validate_position(&candidate) {
                Some(candidate)
            } else {
                None
            }
        }
    }

    fn right(&self, state: &mut State) -> Option<Self> {
        let candidate = Position::new((self.x + 1, self.y), self.equipped);
        if state.validate_position(&candidate) {
            return Some(candidate);
        }
        None
    }
}

struct State {
    depth: usize,
    index_cache: Cache<usize>,
    erosion_cache: Cache<usize>,
    type_cache: Cache<Terrain>,
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

    fn calc_type(&mut self, x: usize, y: usize) -> Terrain {
        if !self.type_cache.contains_key(&(x, y)) {
            let t = match self.calc_erosion_level(x, y) % 3 {
                0 => Rocky,
                1 => Wet,
                2 => Narrow,
                _ => unreachable!(),
            };
            self.type_cache.insert((x, y), t);
        }
        self.type_cache[&(x, y)]
    }

    fn validate_position(&mut self, position: &Position) -> bool {
        let terrain = self.calc_type(position.x, position.y);
        terrain.is_valid(position.equipped)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heuristic_cost_estimate() {
        assert_eq!(
            0,
            heuristic_cost_estimate(&Position::new((0, 0), Torch), &Position::new((0, 0), Torch))
        );
        assert_eq!(
            1,
            heuristic_cost_estimate(&Position::new((0, 0), Torch), &Position::new((0, 1), Torch))
        );
        assert_eq!(
            2,
            heuristic_cost_estimate(&Position::new((0, 0), Torch), &Position::new((1, 1), Torch))
        );
        assert_eq!(
            10,
            heuristic_cost_estimate(
                &Position::new((10, 0), Torch),
                &Position::new((1, 1), Torch)
            )
        );
        assert_eq!(
            5,
            heuristic_cost_estimate(
                &Position::new((7, 7), Torch),
                &Position::new((10, 5), Torch)
            )
        );
    }

    #[test]
    fn test_get_neighbors() {
        let mut state = State::new(510, (10, 10));

        assert_eq!(
            vec![
                Position::new((0, 0), ClimbingGear),
                Position::new((0, 1), Torch)
            ],
            get_neighbors(&mut state, &Position::new((0, 0), Torch)).as_slice()
        );

        assert_eq!(
            vec![
                Position::new((0, 0), Torch),
                Position::new((0, 1), ClimbingGear),
                Position::new((1, 0), ClimbingGear),
            ],
            get_neighbors(&mut state, &Position::new((0, 0), ClimbingGear)).as_slice()
        );

        assert_eq!(
            vec![
                Position::new((2, 2), Neither),
                Position::new((2, 1), ClimbingGear),
                Position::new((1, 2), ClimbingGear),
            ],
            get_neighbors(&mut state, &Position::new((2, 2), ClimbingGear)).as_slice()
        );

        assert_eq!(
            vec![
                Position::new((2, 2), ClimbingGear),
                Position::new((2, 1), Neither),
                Position::new((2, 3), Neither),
                Position::new((1, 2), Neither),
                Position::new((3, 2), Neither),
            ],
            get_neighbors(&mut state, &Position::new((2, 2), Neither)).as_slice()
        );

        assert_eq!(
            vec![
                Position::new((3, 1), Neither),
                Position::new((3, 0), Torch),
                Position::new((3, 2), Torch),
            ],
            get_neighbors(&mut state, &Position::new((3, 1), Torch)).as_slice()
        );

        assert_eq!(
            vec![
                Position::new((3, 1), Torch),
                Position::new((3, 0), Neither),
                Position::new((3, 2), Neither),
                Position::new((2, 1), Neither),
                Position::new((4, 1), Neither),
            ],
            get_neighbors(&mut state, &Position::new((3, 1), Neither)).as_slice()
        );
    }
}
