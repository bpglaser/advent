use std::collections::HashMap;

fn main() {
    let grid = generate_grid(25);
    println!("{:?}", grid);
}

fn generate_grid(target: usize) -> HashMap<u32, (i32, i32)> {
    let mut grid = HashMap::with_capacity(target);
    grid.insert(1, (0, 0));

    let mut x = 1;
    let mut y = 0;
    let mut side = 0;
    let mut side_len = 3;
    let mut count = 1;

    for n in 2..target + 1 {
        grid.insert(n as u32, (x, y));
        println!("[{}]\t({}, {}) => side={}\tside_len={}\tcount={}", n, x, y, side, side_len, count);

        count += 1;
        if count == side_len - 1 {
            side = (side + 1) % 4;
            if side == 0 {
                count = 1;
                side_len += 2;
            } else {
                count = 0;
            }
        }

        match side {
            0 => y -= 1,
            1 => x -= 1,
            2 => y += 1,
            3 => x += 1,
            _ => unreachable!(),
        }
    }

    grid
}
