use std::env::args;

fn main() {
    let (target_x, target_y) = get_input();
    let mut cell_value: u64 = 20151125;
    let mut x = 1;
    let mut y = 1;

    while x != target_x || y != target_y  {
        y -= 1;
        x += 1;

        if y == 0 {
            y = x;
            x = 1;
        }

        cell_value = (cell_value * 252533) % 33554393;
    }

    println!("Answer: {}", cell_value);
}

fn get_input() -> (u64, u64) {
    let inputs: Vec<u64> = args().skip(1).map(|s| s.parse().unwrap()).collect();
    (inputs[0], inputs[1])
}