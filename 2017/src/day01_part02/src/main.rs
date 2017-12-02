const INPUT: &str = include_str!("input.txt");

fn main() {
    let chars: Vec<char> = INPUT.trim().chars().collect();
    let offset = chars.len() / 2;

    let mut sum = 0u32;

    for (i, c) in chars.iter().enumerate() {
        let next = &chars[(i + offset) % chars.len()];
        if c == next {
            sum += c.to_string().parse::<u32>().unwrap();
        }
    }
    println!("answer: {}", sum);
}
