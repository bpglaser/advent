use std::env::args;
use std::fs::File;
use std::io::Read;

fn main() {
    let lines = load_lines();

    let mut total_actual_len = 0;
    let mut total_parsed_len = 0;

    for line in lines.iter() {
        let (actual_len, parsed_len) = get_lengths(&line);
        println!("{} => {} {}", line, actual_len, parsed_len);
        total_actual_len += actual_len;
        total_parsed_len += parsed_len;
    }

    let answer = total_actual_len - total_parsed_len;
    println!("Answer: {}", answer);
}

fn get_lengths(s: &str) -> (isize, isize) {
    let actual_len = s.len() as isize;
    let mut parsed_len = 0;

    if actual_len - 2 > 0 {
        let mut chars = s.chars().skip(1).take(actual_len as usize - 2).peekable();
        let mut skip_count = 0;

        while let Some(c) = chars.next() {
            if skip_count > 0 {
                skip_count -= 1;
                continue;
            }

            if c != '\\' {
                parsed_len += 1;
            } else {
                if let Some(next) = chars.peek() {
                    match next {
                        &'\"' | &'\\' => {
                            parsed_len += 1;
                            skip_count = 1;
                        }
                        &'x' => skip_count = 2,
                        _ => skip_count = 1,
                    }
                }
            }
        }
    }


    (actual_len, parsed_len)
}

fn load_lines() -> Vec<String> {
    let mut file = File::open(args().nth(1).unwrap()).unwrap();
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();
    buf.lines().map(|s| s.trim().to_owned()).collect()
}
