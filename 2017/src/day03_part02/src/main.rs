use std::collections::HashMap;
use std::env::args;

// 283758 too high

fn main() {
    let input = args().nth(1).unwrap().parse().unwrap();
    let answer = find_answer(input);
    println!("answer: {}", answer);
}

fn find_answer(target: usize) -> u32 {
    let mut grid = HashMap::with_capacity(target);
    grid.insert((0, 0), 1);

    let mut x = 1i32;
    let mut y = 0i32;
    let mut side = 0;

    loop {
        let n = get_adjacent(&grid, x, y);
        if n > target as u32 {
            return n;
        }

        grid.insert((x, y), n);
        println!("{:?} => {}", (x, y), n);

        let left = match side {
            0 => (x - 1, y),
            1 => (x, y + 1),
            2 => (x + 1, y),
            3 => (x, y - 1),
            _ => unreachable!(),
        };

        if grid.contains_key(&left) {
            // move forward
            match side {
                0 => y -= 1,
                1 => x -= 1,
                2 => y += 1,
                3 => x += 1,
                _ => unreachable!(),
            }
        } else {
            // turn left
            side = (side + 1) % 4;

            // move forward
            match side {
                0 => y -= 1,
                1 => x -= 1,
                2 => y += 1,
                3 => x += 1,
                _ => unreachable!(),
            }
        }
    }
}

fn get_adjacent(grid: &HashMap<(i32, i32), u32>, x: i32, y: i32) -> u32 {
    let ul = grid.get(&(x - 1, y - 1)).unwrap_or(&0);
    let u = grid.get(&(x, y - 1)).unwrap_or(&0);
    let ur = grid.get(&(x + 1, y - 1)).unwrap_or(&0);

    let l = grid.get(&(x - 1, y)).unwrap_or(&0);
    let r = grid.get(&(x + 1, y)).unwrap_or(&0);

    let dl = grid.get(&(x - 1, y + 1)).unwrap_or(&0);
    let d = grid.get(&(x, y + 1)).unwrap_or(&0);
    let dr = grid.get(&(x + 1, y + 1)).unwrap_or(&0);

    ul + u + ur + l + r + dl + d + dr
}
