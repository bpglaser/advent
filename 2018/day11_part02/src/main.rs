use std::env::args;

fn main() {
    let serial_number: i32 = args().skip(1).next().map(|s| s.parse().unwrap()).unwrap();
    let (x, y, size, power) = solve(serial_number);
    println!("{},{},{} => {}", x, y, size, power);
}

fn solve(serial_number: i32) -> (usize, usize, usize, i32) {
    let mut grid = vec![vec![vec![0; 301]; 301]; 301];

    for y in 1..=300 {
        for x in 1..=300 {
            grid[1][y][x] = power_level(x as i32, y as i32, serial_number);
        }
    }

    let mut ans = (0, 0, 0, i32::min_value());
    for size in 2..=300 {
        for y in 1..=(300 - size) {
            for x in 1..=(300 - size) {
                let point = grid[1][y][x];
                let subsquare = grid[size - 1][y + 1][x + 1];

                let mut row = 0;
                let mut col = 0;
                for offset in 1..size {
                    row += grid[1][y][x + offset];
                    col += grid[1][y + offset][x];
                }

                let power = point + subsquare + row + col;
                grid[size][y][x] = power;

                if power > ans.3 {
                    ans = (x, y, size, power);
                }
            }
        }
    }

    return ans;
}

fn power_level(x: i32, y: i32, serial_number: i32) -> i32 {
    let rack_id = x + 10;
    let mut pl = rack_id * y;
    pl += serial_number;
    pl *= rack_id;
    pl /= 100;
    pl %= 10;
    pl - 5
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_power_levels() {
        assert_eq!(-5, power_level(122, 79, 57));
        assert_eq!(0, power_level(217, 196, 39));
        assert_eq!(4, power_level(101, 153, 71));
    }

    #[test]
    fn given_answers() {
        assert_eq!((90, 269, 16, 113), solve(18));
        assert_eq!((232, 251, 12, 119), solve(42));
    }
}
