use std::env::args;
use std::fmt;
use std::fs::File;
use std::io::Read;

const DEFAULT_ITERATION_COUNT: u32 = 5;

fn main() {
    let path = args().nth(1).expect("input path");
    let iteration_count = args().nth(2).and_then(|s| s.parse().ok()).unwrap_or(DEFAULT_ITERATION_COUNT);

    let rules = load_input(&path);
    println!("Loaded {} rules from {}", rules.len(), path);

    let mut grid = Grid::starting_pattern();

    println!("===== [ {} ] =====\n", 0);
    println!("{}", grid);

    for i in 0..iteration_count {
        let mut all_enhanced = vec![];
        for sub_grid in grid.subdivide() {
            let enhanced_sub_grid = sub_grid.enhance(&rules);
            all_enhanced.push(enhanced_sub_grid);
        }
        grid = Grid::from_sub_grids(&all_enhanced);

        println!("===== [ {} ] =====\n", i + 1);
        println!("{}", grid);
    }
    println!("Lit pixels: {}", grid.count_hash_pixels());
}

fn load_input(path: &str) -> Vec<Rule> {
    let mut file = File::open(path).unwrap();
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();
    buf.trim().lines().map(Rule::from).collect()
}

#[derive(Debug)]
struct Rule {
    predicate: Grid,
    output: Grid,
}

impl Rule {
    fn matches(&self, grid: &Grid) -> bool {
        // check given grid
        if grid == &self.predicate {
            return true;
        }

        // check 90, 180, 270 degree rotations
        if self.match_grid_rotations(grid) {
            return true;
        }

        // check flipped grid
        let flipped_grid = grid.flip();
        if flipped_grid == self.predicate {
            return true;
        }

        // check rotations of flipped grid
        self.match_grid_rotations(&flipped_grid)
    }

    fn match_grid_rotations(&self, grid: &Grid) -> bool {
        let mut rotated_grid = grid.rotate();
        for _ in 0..3 { 
            if rotated_grid == self.predicate {
                return true;
            }
            rotated_grid = rotated_grid.rotate();
        }
        false
    }
}

impl<'a> From<&'a str> for Rule {
    fn from(s: &str) -> Self {
        let mut split = s.split(" => ").map(Grid::from);
        let predicate = split.next().unwrap();
        let output = split.next().unwrap();
        Self { predicate, output }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Grid {
    size: usize,
    pixels: Vec<Pixel>,
}

impl Grid {
    fn starting_pattern() -> Self {
        Grid::from(".#./..#/###")
    }

    fn default_of_size(size: usize) -> Self {
        let pixels = (0..size * size).map(|_| Pixel::Dot).collect();
        Grid { size, pixels }
    }

    fn from_sub_grids(sub_grids: &[Grid]) -> Self {
        let sub_grid_size = sub_grids[0].size;
        assert!(sub_grids.iter().skip(1).all(|grid| sub_grid_size == grid.size));

        let sub_grids_width = (sub_grids.len() as f64).sqrt() as usize;
        let size = sub_grids_width * sub_grid_size;

        let mut pixels = vec![];

        for y in 0..size {
            for x in 0..size {
                let sub_grid = &sub_grids[(x / sub_grid_size) + (y / sub_grid_size) * sub_grids_width];
                let pixel = sub_grid.get(x % sub_grid_size, y % sub_grid_size);
                pixels.push(*pixel);
            }
        }

        Grid { size, pixels }
    }

    fn get(&self, x: usize, y: usize) -> &Pixel {
        self.pixels.get(x + y * self.size).expect("get in bounds")
    }

    fn set(&mut self, x: usize, y: usize, pixel: Pixel) {
        *self.pixels.get_mut(x + y * self.size).expect("set in bounds") = pixel;
    }

    fn subdivide(&self) -> Vec<Self> {
        let sub_grid_size = if self.size % 2 == 0 {
            2
        } else if self.size % 3 == 0 {
            3
        } else {
            panic!("grid size should always be divisible by 2 or 3")
        };

        let number_of_sub_grids_per_width = self.size / sub_grid_size;
        let total_number_of_sub_grids = number_of_sub_grids_per_width * number_of_sub_grids_per_width;
        let mut sub_grids: Vec<Grid> = (0..total_number_of_sub_grids).map(|_| Grid::default_of_size(sub_grid_size)).collect();
        
        for y in 0..self.size {
            for x in 0..self.size {
                let sub_grid = &mut sub_grids[(x / sub_grid_size) + (y / sub_grid_size) * number_of_sub_grids_per_width];
                let pixel = self.get(x, y);
                sub_grid.set(x % sub_grid_size, y % sub_grid_size, *pixel);
            }
        }

        sub_grids
    }

    fn enhance(&self, rules: &[Rule]) -> Self {
        match rules.iter().find(|rule| rule.matches(self)) {
            None => panic!("failed to find a matching rule for:\n{}\nfrom rules:\n{:?}", self, rules),
            Some(rule) => rule.output.clone(),
        }
    }

    fn rotate(&self) -> Self {
        let mut new_grid = self.clone();

        for y in 0..self.size {
            for x in 0..self.size {
                new_grid.set(y, x, *self.get(self.size - 1 - x, y));
            }
        }

        new_grid
    }

    fn flip(&self) -> Self {
        let mut new_grid = self.clone();
        for y in 0..self.size {
            for x in 0..self.size {
                let pixel = self.get(self.size - 1 - x, y);
                new_grid.set(x, y, *pixel);
            }
        }
        new_grid
    }

    fn count_hash_pixels(&self) -> usize {
        self.pixels.iter().filter(|pixel| pixel == &&Pixel::Hash).count()
    }
}

impl<'a> From<&'a str> for Grid {
    fn from(s: &str) -> Self {
        let mut size = None;
        let mut pixels = vec![];
        for (i, token) in s.trim().chars().map(Token::from).enumerate() {
            match token {
                Token::Pixel(p) => pixels.push(p),
                Token::Slash => {
                    if size.is_none() {
                        size = Some(i + 1);
                    } else {
                        assert!((i + 1) % size.unwrap() == 0, "encountered uneven row at {} in {}. size is {:?}", i, s, size);
                    }
                }
            }
        }
        Grid { size: size.expect("valid size") - 1, pixels }
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for chunk in self.pixels.chunks(self.size) {
            for pixel in chunk {
                write!(f, "{}", pixel)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Pixel {
    Hash,
    Dot,
}

impl fmt::Display for Pixel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}",
            match *self {
                Pixel::Hash => '#',
                Pixel::Dot => '.',
            }
        )
    }
}

enum Token {
    Pixel(Pixel),
    Slash,
}

impl From<char> for Token {
    fn from(c: char) -> Self {
        match c {
            '#' => Token::Pixel(Pixel::Hash),
            '.' => Token::Pixel(Pixel::Dot),
            '/' => Token::Slash,
            _ => panic!("invalid token: {}", c),
        }
    }
}

#[test]
fn test_grid_from() {
    use Pixel::*;
    let grid = Grid::from("../.#");
    assert_eq!(Grid { size: 2, pixels: vec![Dot, Dot, Dot, Hash] }, grid);
    
    let grid = Grid::from(".#./..#/###");
    assert_eq!(Grid { size: 3, pixels: vec![Dot, Hash, Dot, Dot, Dot, Hash, Hash, Hash, Hash] }, grid);

    let grid = Grid::from("#..#/..../#..#/.##.");
    assert_eq!(Grid { size: 4, pixels: vec![Hash, Dot, Dot, Hash, Dot, Dot, Dot, Dot, Hash, Dot, Dot, Hash, Dot, Hash, Hash, Dot] }, grid);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let grid = Grid::starting_pattern();
        assert_eq!(Grid::from(".#./..#/###"), grid);
    }

    #[test]
    fn test_rotate() {
        let grid = Grid::starting_pattern();
        assert_eq!(Grid::from(".##/#.#/..#"), grid.rotate()); // 90 ccw
        assert_eq!(Grid::from("###/#../.#."), grid.rotate().rotate()); // 180 ccw
        assert_eq!(Grid::from("#../#.#/##."), grid.rotate().rotate().rotate()); // 270 ccw
        assert_eq!(grid, grid.rotate().rotate().rotate().rotate()); // 360 ccw; back to original

        let grid = Grid::from(".#.#/#..#/#.#./.##.");
        assert_eq!(Grid::from("##../..##/#..#/.##."), grid.rotate()); // 90 ccw
        assert_eq!(Grid::from(".##./.#.#/#..#/#.#."), grid.rotate().rotate()); // 180 ccw
        assert_eq!(Grid::from(".##./#..#/##../..##"), grid.rotate().rotate().rotate()); // 270 ccw
        assert_eq!(grid, grid.rotate().rotate().rotate().rotate()); // 360 ccw; back to original
    }

    #[test]
    fn test_flip() {
        let grid = Grid::starting_pattern();
        assert_eq!(Grid::from(".#./#../###"), grid.flip());
        assert_eq!(grid, grid.flip().flip());

        let grid = Grid::from(".#.#/#..#/#.#./.##.");
        assert_eq!(Grid::from("#.#./#..#/.#.#/.##."), grid.flip());
        assert_eq!(grid, grid.flip().flip());
    }

    #[test]
    fn test_from_sub_grids() {
        let sub_grid = Grid::from("##./#../...");
        let sub_grids = vec![sub_grid; 4];
        let merged = Grid::from_sub_grids(&sub_grids);
        assert_eq!(Grid::from("##.##./#..#../....../##.##./#..#../......"), merged);
    }

    #[test]
    fn test_subdivide() {
        // size 2
        let grid = Grid::from("#./.#");
        assert_eq!(vec![grid.clone()], grid.subdivide(), "subdivide size 2");

        // size 3
        let grid = Grid::starting_pattern();
        assert_eq!(vec![Grid::starting_pattern()], grid.subdivide(), "subdivide size 3");

        let grid = Grid::from("#..#/..../..../#..#");
        let given = vec![
            Grid::from("#./.."),
            Grid::from(".#/.."),
            Grid::from("../#."),
            Grid::from("../.#"),
        ];
        assert_eq!(given, grid.subdivide(), "subdivide size 4");
    }
}