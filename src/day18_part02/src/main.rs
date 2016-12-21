use std::env::args;

fn main() {
    let (row_count, mut prev_row) = get_input();

    let mut count = count_safe_tiles(&prev_row);

    for _ in 1..row_count {
        let next = gen_row(&prev_row);
        count += count_safe_tiles(&next);
        prev_row = next;
    }

    println!("Safe tiles: {}", count);
}

fn gen_row(previous: &str) -> String {
    let mut row = String::new();

    let chars: Vec<char> = previous.chars().collect();

    for i in 0..chars.len() {
        let mut left = '.';
        if (i as isize) - 1 >= 0 {
            left = previous.chars().nth(i - 1).unwrap();
        }
        let right = previous.chars().nth(i + 1).unwrap_or('.');

        if (left == '^') ^ (right == '^') {
            row.push('^');
        } else {
            row.push('.');
        }
    }

    row
}

fn count_safe_tiles(row: &str) -> usize {
    let mut count = 0;

    for c in row.chars() {
        if c == '.' {
            count += 1;
        }
    }

    count
}

fn get_input() -> (usize, String) {
    let args: Vec<_> = args().skip(1).collect();
    (args[0].parse().unwrap(), args[1].to_owned())
}
