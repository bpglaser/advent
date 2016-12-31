use std::env::args;

fn main() {
    let (count, input) = read_args();
    let mut previous = input;
    for _ in 0..count {
        let next = look_and_say(&previous);
        previous = next;
    }
    println!("Answer length: {}", previous.len());
}

fn look_and_say(s: &str) -> String {
    let mut result = String::new();
    let mut chars = s.chars().peekable();

    let mut count = 1;
    while let Some(current) = chars.next() {
        match chars.peek() {
            Some(next) => {
                if next == &current {
                    count += 1;
                } else {
                    result.push_str(&count.to_string());
                    result.push(current);
                    count = 1;
                }
            }
            None => {
                result.push_str(&count.to_string());
                result.push(current);
            }
        }
    }

    result
}

fn read_args() -> (usize, String) {
    let args: Vec<String> = args().skip(1).collect();
    (args[0].parse().unwrap(), args[1].to_owned())
}
