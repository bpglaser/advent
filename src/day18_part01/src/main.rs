use std::env::args;

fn main() {
    let (row_count, first_row) = get_input();
    let mut rows = vec![first_row];

    for _ in 1..row_count {
        let next = gen_row(&rows.last().unwrap());
        rows.push(next);
    }

    let count = count_safe_tiles(&rows);
    println!("Safe tiles: {}", count);
}

fn gen_row(previous: &str) -> String {
    let mut row = String::new();

    for (i, center) in previous.char_indices() {
        let mut left = '.';
        if i as isize - 1 >= 0 {
        left = previous.chars().nth(i - 1).unwrap_or('.');
        }
        let right = previous.chars().nth(i + 1).unwrap_or('.');

        if left == '^' && center == '^' && right == '.' {
            row.push('^');
        } else if left == '.' && center == '^' && right == '^' {
            row.push('^');
        } else if left == '^' && center == '.' && right == '.' {
            row.push('^');
        } else if left == '.' && center == '.' && right == '^' {
            row.push('^');
        } else {
            row.push('.');
        }
    }

    row
}

fn count_safe_tiles(rows: &Vec<String>) -> usize {
    let mut count = 0;

    for row in rows {
        for c in row.chars() {
            if c == '.' {
                count += 1;
            }
        }
    }

    count
}

fn get_input() -> (usize, String) {
    let args: Vec<_> = args().skip(1).collect();
    (args[0].parse().unwrap(), args[1].to_owned())
}
