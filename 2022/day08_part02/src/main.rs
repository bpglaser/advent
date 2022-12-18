use std::{env::args, fs::read_to_string, ops::Mul};

const OFFSETS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

#[derive(Debug)]
struct Grid {
    values: Vec<Vec<u8>>,
    scores: Vec<Vec<[usize; 4]>>,
}

impl Grid {
    fn new(s: &str) -> Self {
        let values: Vec<Vec<u8>> = s
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap() as u8)
                    .collect()
            })
            .collect();
        let w = values[0].len();
        let h = values.len();
        Self {
            values,
            scores: vec![vec![[0; 4]; w]; h],
        }
    }

    fn size(&self) -> (usize, usize) {
        (self.values[0].len(), self.values.len())
    }

    fn get(&self, x: usize, y: usize) -> u8 {
        self.values[y][x]
    }

    fn try_get(&self, x: i32, y: i32) -> Option<u8> {
        let (w, h) = self.size();
        if x < 0 || x >= w as i32 || y < 0 || y >= h as i32 {
            return None;
        }
        Some(self.values[y as usize][x as usize])
    }

    fn mark(&mut self, x: usize, y: usize, dir: usize, val: usize) {
        self.scores[y][x][dir] = val;
    }

    fn get_score(&self, x: usize, y: usize) -> usize {
        self.scores[y][x].iter().fold(1, Mul::mul)
    }

    fn score(&mut self, x: usize, y: usize) {
        let h = self.get(x, y);
        for (dir, (dx, dy)) in OFFSETS.into_iter().enumerate() {
            let (mut working_x, mut working_y) = (x as i32 + dx, y as i32 + dy);
            let mut dist = 1;
            loop {
                match self.try_get(working_x, working_y) {
                    Some(found_h) if found_h >= h => {
                        // found a blocking tree;
                        self.mark(x, y, dir, dist);
                        break;
                    }
                    None => {
                        // No blocking trees between me and the edge.
                        self.mark(x, y, dir, dist - 1);
                        break;
                    }
                    _ => {} // Not blocking, continue;
                }
                working_x += dx;
                working_y += dy;
                dist += 1;
            }
        }
    }

    fn score_all(&mut self) {
        let (w, h) = self.size();

        for y in 1..h - 1 {
            for x in 1..w - 1 {
                self.score(x, y);
            }
        }
    }

    fn best(&self) -> (usize, usize, usize) {
        let (w, h) = self.size();
        let mut best = None;
        for y in 1..h - 1 {
            for x in 1..w - 1 {
                let score = self.get_score(x, y);
                match best {
                    None => {
                        best = Some((x, y, score));
                        continue;
                    }
                    Some((_, _, best_score)) => {
                        if score > best_score {
                            best = Some((x, y, score));
                        }
                    }
                }
            }
        }
        best.expect("no best :(")
    }
}

fn solve(s: &str) -> String {
    let mut grid = Grid::new(s);
    grid.score_all();
    format!("{}", grid.best().2)
}

fn input() -> String {
    let path = args().nth(1).unwrap();
    read_to_string(path).unwrap()
}

fn main() {
    let s = input();
    let res = solve(&s);
    println!("{}", res);
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    const GIVEN: &str = "30373
25512
65332
33549
35390";

    #[test]
    fn test_scoring() {
        let mut grid = Grid::new(&GIVEN);
        grid.score_all();
        let (w, h) = grid.size();

        let mut actual = vec![vec![0; w]; h];
        for y in 0..h {
            for x in 0..w {
                actual[y][x] = grid.get_score(x, y);
            }
        }

        let expected = vec![
            vec![0, 0, 0, 0, 0],
            vec![0, 1, 4, 1, 0],
            vec![0, 6, 1, 2, 0],
            vec![0, 1, 8, 3, 0],
            vec![0, 0, 0, 0, 0],
        ];
        assert_eq!(expected, actual);
        assert_eq!(grid.best(), (2, 3, 8));
    }

    #[test]
    fn test_given() {
        assert_eq!(solve(&GIVEN), "8");
    }
}
