const INPUT: &str = include_str!("input.txt");

fn main() {
    let input = INPUT.trim();
    let mut iterator = input.chars().cycle().peekable();
    let mut sum = 0u32;
    for _ in 0..input.len() {
        let c = iterator.next().unwrap();
        let next = iterator.peek().unwrap();
        if &c == next {
            sum += c.to_string().parse::<u32>().unwrap();
        }
    }
    println!("answer: {}", sum);
}
