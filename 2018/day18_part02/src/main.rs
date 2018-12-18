use std::collections::{HashMap, HashSet};
use std::env::args;
use std::fs::File;
use std::io::{stdout, Read, Write};

use gif::*;

const DURATION: usize = 1000000000;
type Grid = Vec<Vec<char>>;

fn main() {
    let path = args().nth(1).expect("input path");
    let mut grid = read_input(&path);

    let mut scores = vec![score(&grid)];
    let mut seen = HashMap::new();
    seen.insert(grid.clone(), 0);

    for i in 1..=DURATION {
        grid = step(&grid);
        if seen.contains_key(&grid) {
            let start = seen.get(&grid).unwrap();
            let j = start + ((DURATION - start) % (i - start));
            println!("{:?}", scores[j]);
            output_image(&grid).unwrap();
            return;
        }
        scores.push(score(&grid));
        seen.insert(grid.clone(), i);
    }

    // We realistically shouldn't get here!
    render(&grid, &mut stdout());
    println!("ans: {}", score(&grid));
}

fn output_image(grid: &Grid) -> Result<(), Box<std::error::Error>> {
    let out = File::create("out.gif")?;
    let mut encoder = Encoder::new(out, (grid[0].len() * 10) as u16, (grid.len() * 10) as u16, &[])?;
    Repeat::Infinite.set_param(&mut encoder)?;

    let mut seen = HashSet::new();
    let mut grid = grid.clone();
    while !seen.contains(&grid) {
        let rgb = create_frame(&grid);
        let frame = Frame::from_rgb((grid[0].len() * 10) as u16, (grid.len() * 10) as u16, rgb.as_slice());
        encoder.write_frame(&frame)?;
        let next = step(&grid);
        seen.insert(grid);
        grid = next;
    }

    Ok(())
}

fn create_frame(grid: &Grid) -> Vec<u8> {
    let tan = [0xFF, 0xE9, 0xC7];
    let brown = [0xA3, 0x7C, 0x19];
    let green = [0x39, 0x50, 0x36];

    let mut scaled = vec![vec!['!'; grid[0].len() * 10]; grid.len() * 10];
    for (y, row) in grid.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            for dy in 0..10 {
                for dx in 0..10 {
                    scaled[y * 10 + dy][x * 10 + dx] = *c;
                }
            }
        }
    }

    let mut buf = vec![];

    for row in scaled {
        for c in row {
            buf.extend(match c {
                '.' => tan.iter(),
                '#' => brown.iter(),
                '|' => green.iter(),
                _ => unreachable!(),
            });
        }
    }

    buf
}

fn score(grid: &Grid) -> usize {
    let mut w = 0;
    let mut l = 0;

    for row in grid {
        for c in row {
            match c {
                '.' => {}
                '|' => w += 1,
                '#' => l += 1,
                _ => unreachable!(),
            }
        }
    }

    w * l
}

fn step(grid: &Grid) -> Grid {
    let mut new_grid = vec![];

    for y in 0..grid.len() {
        let mut row = vec![];

        for x in 0..grid[0].len() {
            match grid[y][x] {
                '.' => {
                    if count_adjacent(grid, x, y, '|') >= 3 {
                        row.push('|');
                    } else {
                        row.push('.');
                    }
                }
                '|' => {
                    if count_adjacent(grid, x, y, '#') >= 3 {
                        row.push('#');
                    } else {
                        row.push('|');
                    }
                }
                '#' => {
                    if count_adjacent(grid, x, y, '#') >= 1 && count_adjacent(grid, x, y, '|') >= 1
                    {
                        row.push('#');
                    } else {
                        row.push('.');
                    }
                }
                _ => unreachable!(),
            }
        }

        new_grid.push(row);
    }

    new_grid
}

fn count_adjacent(grid: &Grid, x: usize, y: usize, c: char) -> usize {
    let mut count = 0;

    for dy in -1..=1 {
        for dx in -1..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }

            let x = x as isize + dx;
            let y = y as isize + dy;

            if x < 0 || y < 0 || x as usize >= grid[0].len() || y as usize >= grid.len() {
                continue;
            }

            if grid[y as usize][x as usize] == c {
                count += 1;
            }
        }
    }

    count
}

fn render<T: Write>(grid: &Grid, w: &mut T) {
    for line in grid {
        for c in line {
            write!(w, "{}", c).unwrap();
        }
        writeln!(w).unwrap();
    }
}

fn read_input(path: &str) -> Grid {
    let mut file = File::open(path).unwrap();
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();
    buf.lines().map(|line| line.chars().collect()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count() {
        let grid = vec![
            vec!['.', '#', '.', '#', '.', '.', '.', '|', '#', '.'],
            vec!['.', '.', '.', '.', '.', '#', '|', '#', '#', '|'],
            vec!['.', '|', '.', '.', '|', '.', '.', '.', '#', '.'],
            vec!['.', '.', '|', '#', '.', '.', '.', '.', '.', '#'],
            vec!['#', '.', '#', '|', '|', '|', '#', '|', '#', '|'],
            vec!['.', '.', '.', '#', '.', '|', '|', '.', '.', '.'],
            vec!['.', '|', '.', '.', '.', '.', '|', '.', '.', '.'],
            vec!['|', '|', '.', '.', '.', '#', '|', '.', '#', '|'],
            vec!['|', '.', '|', '|', '|', '|', '.', '.', '|', '.'],
            vec!['.', '.', '.', '#', '.', '|', '.', '.', '|', '.'],
        ];
        assert_eq!(3, count_adjacent(&grid, 7, 0, '#'));
    }
}
