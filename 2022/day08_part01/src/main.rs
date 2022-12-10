use std::{env::args, fmt, fs::read_to_string};

struct Grid(Vec<Vec<u8>>);

impl Grid {
    fn new(s: &str) -> Self {
        let mut grid = Self(
            s.lines()
                .map(|line| {
                    line.chars()
                        .map(|c| c.to_digit(10).unwrap() as u8)
                        .collect()
                })
                .collect(),
        );
        let (w, h) = grid.size();
        for x in 0..w {
            grid.mark(x, 0);
            grid.mark(x, h - 1);
        }
        for y in 1..h - 1 {
            grid.mark(0, y);
            grid.mark(w - 1, y);
        }
        grid
    }

    fn size(&self) -> (usize, usize) {
        (self.0[0].len(), self.0.len())
    }

    fn get(&self, x: usize, y: usize) -> u8 {
        self.0[y][x] & 0b0111_1111
    }

    fn mark(&mut self, x: usize, y: usize) {
        self.0[y][x] |= 0b1000_0000;
    }

    fn is_marked(&self, x: usize, y: usize) -> bool {
        self.0[y][x] & 0b1000_0000 != 0
    }

    fn marked_count(&self) -> usize {
        let (w, h) = self.size();
        let mut count = 0;
        for y in 0..h {
            for x in 0..w {
                if self.is_marked(x, y) {
                    count += 1;
                }
            }
        }
        count
    }
}

impl fmt::Debug for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (w, h) = self.size();
        for y in 0..h {
            for x in 0..w {
                let i = self.get(x, y);
                if self.is_marked(x, y) {
                    write!(f, "[{}]", i)?;
                } else {
                    write!(f, " {} ", i)?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn solve(s: &str) -> String {
    let mut grid = Grid::new(s);
    let (w, h) = grid.size();

    // Left to right
    for y in 1..h - 1 {
        let mut tallest = grid.get(0, y);
        for x in 1..w - 1 {
            let i = grid.get(x, y);
            if i > tallest {
                tallest = i;
                grid.mark(x, y);
            }
        }
    }

    // Right to left
    for y in 1..h - 1 {
        let mut tallest = grid.get(w - 1, y);
        for x in (1..w - 1).rev() {
            let i = grid.get(x, y);
            if i > tallest {
                tallest = i;
                grid.mark(x, y);
            }
        }
    }

    // Top to bottom
    for x in 1..w - 1 {
        let mut tallest = grid.get(x, 0);
        for y in 1..h - 1 {
            let i = grid.get(x, y);
            if i > tallest {
                tallest = i;
                grid.mark(x, y);
            }
        }
    }

    // Bottom to top
    for x in 1..w - 1 {
        let mut tallest = grid.get(x, h - 1);
        for y in (1..h - 1).rev() {
            let i = grid.get(x, y);
            if i > tallest {
                tallest = i;
                grid.mark(x, y);
            }
        }
    }

    format!("{}", grid.marked_count())
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
    use super::*;

    #[test]
    fn test_given() {
        let given = "30373
25512
65332
33549
35390";
        assert_eq!(solve(&given), "21");
    }
}
