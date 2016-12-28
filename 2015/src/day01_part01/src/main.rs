fn main() {
    let mut sum: isize = 0;
    for c in std::env::args().skip(1).next().expect("Invalid args").chars() {
        match c {
            '(' => sum += 1,
            ')' => sum -= 1,
            _ => panic!("Invalid input"),
        }
    }
    println!("Floor: {}", sum);
}
