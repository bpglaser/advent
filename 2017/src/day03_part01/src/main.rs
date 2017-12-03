use std::collections::HashMap;
use std::env::args;

fn main() {
    let input = args().nth(1).unwrap().parse().unwrap();
    let answer = find_answer(input);
    println!("answer: {}", answer);
}

fn find_answer(target: usize) -> u32 {
    let mut grid = HashMap::with_capacity(target);
    grid.insert((0, 0), 1);

    let mut n = 2;
    let mut x = 1i32;
    let mut y = 0i32;
    let mut side = 0;

    loop {
        if n == target {
            return (x.abs() + y.abs()) as u32;
        }
        grid.insert((x, y), n);

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

        n += 1;
    }
}
