use std::collections::VecDeque;
use std::env::args;
use std::fs::File;
use std::io::Read;
use std::mem;

fn main() {
    let path = args().nth(1).unwrap();
    let mut buf = String::new();
    let mut file = File::open(path).unwrap();
    file.read_to_string(&mut buf).unwrap();
    println!("{}", solve(&buf));
}

fn solve(input: &str) -> i32 {
    let mut rounds = 0;
    let mut grid = build_grid(input);
    println!("Initially:");
    render(&grid);
    println!();

    loop {
        if do_round(&mut grid) {
            rounds += 1;
        } else {
            break;
        }

        println!("After {} rounds:", rounds);
        render(&grid);
        println!();
    }

    println!("Final state:");
    render(&grid);
    println!();

    score(rounds, &grid)
}

fn render(grid: &Grid) {
    for row in grid {
        let mut healths = vec![];
        for tile in row {
            match tile {
                Tile::Elf(meta) => {
                    healths.push(format!("E({})", meta.hit_points));
                    print!("E");
                }
                Tile::Goblin(meta) => {
                    healths.push(format!("G({})", meta.hit_points));
                    print!("G");
                }
                Tile::Open => print!("."),
                Tile::Wall => print!("#"),
            }
        }
        println!("    {}", healths.join(", "));
    }
}

fn do_round(grid: &mut Grid) -> bool {
    let mut todo = vec![];
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x].is_unit() {
                todo.push((x, y));
            }
        }
    }

    let num_todo = todo.len();
    for (i, (x, y)) in todo.into_iter().enumerate() {
        println!("working on {:?}", grid[y][x]);
        let (x, y) = do_move(grid, x, y);
        do_attack(grid, x, y);

        let (elves, goblins)= count_units(&grid);
        if elves == 0 || goblins == 0 {
            return i == num_todo - 1;
        }
    }

    true
}

fn do_move(grid: &mut Grid, x: usize, y: usize) -> (usize, usize) {
    println!("moving {:?} at {:?}", grid[y][x], (x, y));
    // Check if we're already adjacent to an enemy.
    for (ax, ay) in get_adjacent(x, y) {
        match (&grid[y][x], &grid[ay][ax]) {
            (Tile::Elf(_), Tile::Goblin(_)) => return (x, y),
            (Tile::Goblin(_), Tile::Elf(_)) => return (x, y),
            _ => {}
        }
    }

    // Find all the enemies.
    let mut enemies = vec![];
    for (ey, row) in grid.iter().enumerate() {
        for (ex, unit) in row.iter().enumerate() {
            match (&grid[y][x], unit) {
                (Tile::Elf(_), Tile::Goblin(_)) => enemies.push((ex, ey)),
                (Tile::Goblin(_), Tile::Elf(_)) => enemies.push((ex, ey)),
                _ => {}
            }
        }
    }

    let mut shortest = vec![];
    // println!("enemies: {:?}", enemies.iter().map(|(x, y)| &grid[*y][*x]).collect::<Vec<_>>());
    // Flood fill from each of the enemies.
    for (ex, ey) in enemies {
        let mut flood = vec![vec![None; grid[0].len()]; grid.len()];
        flood_fill(grid, &mut flood, ex, ey);

        for (ax, ay) in get_adjacent(x, y) {
            if let Some(dist) = flood[ay][ax] {
                shortest.push(((ax, ay), (ex, ey), dist));
            }
        }
    }

    if shortest.is_empty() {
        return (x, y);
    }

    let shortest_dist = shortest.iter().map(|&(_, _, dist)| dist).min().unwrap();
    shortest.retain(|&(_, _, dist)| dist == shortest_dist);
    shortest.sort_by_key(|&((ax, ay), _, _)| (ay, ax));
    let (movx, movy) = shortest[0].0;

    let mut tmp = Tile::Open;
    // Take the tile out of the grid and leave Open in its place
    mem::swap(&mut tmp, &mut grid[y][x]);
    // Restore the tile to the grid
    grid[movy][movx] = tmp;

    (movx, movy)
}

fn flood_fill(grid: &Grid, flood: &mut Vec<Vec<Option<u32>>>, x: usize, y: usize) {
    println!("doing flood fill starting at {:?} [{:?}]", (x, y), grid[y][x]);
    let mut frontier = VecDeque::new();
    frontier.push_back((x, y, 0));

    while let Some((x, y, dist)) = frontier.pop_front() {
        flood[y][x] = Some(dist);

        for (ax, ay) in get_adjacent(x, y) {
            if flood[ay][ax].is_some() {
                continue;
            }

            match &grid[ay][ax] {
                Tile::Open => frontier.push_back((ax, ay, dist + 1)),
                _ => {},
            }
        }
    }
}

fn do_attack(grid: &mut Grid, x: usize, y: usize) {
    println!("attacking with {:?} at {:?}", grid[y][x], (x, y));
    let mut ap = None;
    let mut chosen: Option<(usize, usize, i32)> = None;

    for (ax, ay) in get_adjacent(x, y) {
        let attacker = &grid[y][x];
        let target = &grid[ay][ax];

        match (attacker, target) {
            (Tile::Elf(a), Tile::Goblin(t)) => {
                if chosen.is_none() || t.hit_points < chosen.unwrap().2 {
                    ap = Some(a.attack_power);
                    chosen = Some((ax, ay, t.hit_points));
                }
            }
            (Tile::Goblin(a), Tile::Elf(t)) => {
                if chosen.is_none() || t.hit_points < chosen.unwrap().2 {
                    ap = Some(a.attack_power);
                    chosen = Some((ax, ay, t.hit_points));
                }
            }
            _ => {}
        }
    }

    if let Some((tx, ty, _)) = chosen {
        match &mut grid[ty][tx] {
            Tile::Elf(meta) | Tile::Goblin(meta) => {
                // println!("{:?} is attacking {:?}", (x, y), (tx, ty));
                meta.hit_points -= ap.unwrap();
                if meta.hit_points <= 0 {
                    grid[ty][tx] = Tile::Open;
                }
            }
            _ => unreachable!(),
        }
    }
}

fn get_adjacent(x: usize, y: usize) -> Vec<Coord> {
    assert!(x >= 1 && y >= 1);
    let mut adjacent = vec![];

    for (dx, dy) in &[(0, -1), (-1, 0), (1, 0), (0, 1)] {
        let x = x as isize + dx;
        let y = y as isize + dy;
        adjacent.push((x as usize, y as usize));
    }

    adjacent
}

fn score(rounds: i32, grid: &Grid) -> i32 {
    let mut score = 0;
    for row in grid {
        for tile in row {
            match tile {
                Tile::Elf(meta) => score += meta.hit_points,
                Tile::Goblin(meta) => score += meta.hit_points,
                _ => {}
            }
        }
    }
    rounds * score
}

fn count_units(grid: &Grid) -> (usize, usize) {
    let mut elves = 0;
    let mut goblins = 0;
    for row in grid {
        for unit in row {
            match unit {
                Tile::Elf(_) => elves += 1,
                Tile::Goblin(_) => goblins += 1,
                _ => {}
            }
        }
    }
    (elves, goblins)
}

fn build_grid(input: &str) -> Vec<Vec<Tile>> {
    let mut grid = vec![];

    for (y, line) in input.lines().enumerate() {
        let mut row = vec![];

        for (x, c) in line.chars().enumerate() {
            let tile = match c {
                'E' => Tile::Elf(Meta::from_pos((x, y))),
                'G' => Tile::Goblin(Meta::from_pos((x, y))),
                '.' => Tile::Open,
                '#' => Tile::Wall,
                _ => panic!("invalid tile: {} at {:?}", c, (x, y)),
            };
            row.push(tile);
        }

        grid.push(row);
    }

    grid
}

type Grid = Vec<Vec<Tile>>;
type Coord = (usize, usize);

#[derive(Debug)]
enum Tile {
    Elf(Meta),
    Goblin(Meta),
    Open,
    Wall,
}

impl Tile {
    fn is_unit(&self) -> bool {
        match self {
            Tile::Elf(_) | Tile::Goblin(_) => true,
            _ => false,
        }
    }
}

#[derive(Debug)]
struct Meta {
    attack_power: i32,
    hit_points: i32,
    start_pos: Coord,
}

impl Meta {
    fn from_pos(pos: Coord) -> Self {
        Self {
            attack_power: 3,
            hit_points: 200,
            start_pos: pos,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_given0() {
        let input = include_str!("../given0.txt");
        assert_eq!(27730, solve(&input));
    }

    #[test]
    fn test_given1() {
        let input = include_str!("../given1.txt");
        assert_eq!(36334, solve(&input));
    }

    #[test]
    fn test_given2() {
        let input = include_str!("../given2.txt");
        assert_eq!(39514, solve(&input));
    }

    #[test]
    fn test_given3() {
        let input = include_str!("../given3.txt");
        assert_eq!(27755, solve(&input));
    }

    #[test]
    fn test_given4() {
        let input = include_str!("../given4.txt");
        assert_eq!(28944, solve(&input));
    }

    #[test]
    fn test_given5() {
        let input = include_str!("../given5.txt");
        assert_eq!(18740, solve(&input));
    }
}
