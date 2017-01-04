use std::error::Error;
use std::env::args;
use std::fs::File;
use std::io::Read;

type Grid = Vec<Vec<usize>>;

fn main() {
    let (iteration_count, path) = get_args();
    let mut grid = load_grid(&path).expect("Error loading");

    println!("Initial grid:");
    print_grid(&grid);

    for i in 0..iteration_count {
        let mut to_light = vec![];
        let mut to_darken = vec![];

        for y in 1..(grid.len() - 1) {
            for x in 1..(grid.len() - 1) {
                let neighbors = count_neighbors(&grid, x, y);
                if grid[y][x] > 0 {
                    if neighbors == 2 || neighbors == 3 {
                        to_light.push((x, y));
                    } else {
                        to_darken.push((x, y));
                    }
                } else {
                    if neighbors == 3 {
                        to_light.push((x, y));
                    }
                }
            }
        }

        for (x, y) in to_light.into_iter() {
            grid[y][x] = 1;
        }
        for (x, y) in to_darken.into_iter() {
            grid[y][x] = 0;
        }

        // println!("After {} iterations:\n", i + 1);
        // print_grid(&grid);
        // println!();
    }

    println!("Result grid:");
    print_grid(&grid);

    let answer = count_lit(&grid);
    println!("Lit count: {}", answer);
}

fn print_grid(grid: &Grid) {
    for grid_line in grid.iter() {
        for point in grid_line.iter() {
            if *point > 0 {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn count_lit(grid: &Grid) -> usize {
    let mut count = 0;
    for grid_line in grid.iter() {
        for point in grid_line.iter() {
            if *point > 0 {
                count += 1;
            }
        }
    }
    count
}

fn count_neighbors(grid: &Grid, x: usize, y: usize) -> usize {
    let mut count = 0;
    for dy in (y - 1)..(y + 2) {
        for dx in (x - 1)..(x + 2) {
            if x == dx && y == dy {
                continue;
            }
            if grid[dy][dx] > 0 {
                count += 1;
            }
        }
    }
    count
}

fn load_grid(path: &str) -> Result<Grid, Box<Error>>{
    let mut grid = vec![];

    for line in load_lines(path)? {
        let mut grid_line = vec![0]; // padding

        for c in line.chars() {
            if c == '#' {
                grid_line.push(1);
            } else {
                grid_line.push(0);
            }
        }

        grid_line.push(0); // padding
        grid.push(grid_line);
    }

    // Inserts padding rows at top / bottom of the grid
    let size = grid.len();
    grid.insert(0, vec![0; size + 2]);
    grid.push(vec![0; size + 2]);

    Ok(grid)
}

fn load_lines(path: &str) -> Result<Vec<String>, Box<Error>> {
    let mut file = File::open(path)?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;
    Ok(buf.trim().lines().map(|s| s.to_owned()).collect())
}

fn get_args() -> (usize, String) {
    (args().nth(1).unwrap().parse().unwrap(), args().nth(2).unwrap())
}
