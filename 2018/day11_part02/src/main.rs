use std::env::args;

fn main() {
    let serial_number: i32 = args().skip(1).next().map(|s| s.parse().unwrap()).unwrap();
    let (x, y, size, power) = solve(serial_number);
    println!("{},{},{} => {}", x, y, size, power);
}

fn solve(serial_number: i32) -> (usize, usize, usize, i32) {
    let mut grid = vec![vec![0; 301]; 301];
    let mut subsquares = vec![vec![vec![0; 301]; 301]; 2];
    let mut subrows = vec![vec![vec![0; 301]; 301]; 2];
    let mut subcols = vec![vec![vec![0; 301]; 301]; 2];
    let mut ans = (0, 0, 0, i32::min_value());

    for y in 1..=300 {
        for x in 1..=300 {
            let power = power_level(x as i32, y as i32, serial_number);
            grid[y][x] = power;
            subsquares[1][y][x] = power;
            subrows[1][y][x] = power;
            subcols[1][y][x] = power;

            if power > ans.3 {
                ans = (x, y, 1, power);
            }
        }
    }

    for size in 2..=300 {
        for y in 1..=(300 - size) {
            for x in 1..=(300 - size) {
                let point = grid[y][x];
                let subsquare = subsquares[(size - 1) % 2][y + 1][x + 1];

                let row = subrows[(size - 1) % 2][y][x + 1];
                subrows[size % 2][y][x] = point + row;
                let col = subcols[(size - 1) % 2][y + 1][x];
                subcols[size % 2][y][x] = point + col;

                let power = point + subsquare + row + col;
                subsquares[size % 2][y][x] = power;

                if power > ans.3 {
                    ans = (x, y, size, power);
                }
            }
        }
    }

    ans
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
