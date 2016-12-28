fn main() {
    let mut sum: isize = 0;
    for (n, c) in std::env::args().skip(1).next().expect("Invalid args").chars().enumerate() {
        match c {
            '(' => sum += 1,
            ')' => {
                sum -= 1;
                if sum < 0 {
                    println!("Entered the basement: {}", n + 1);
                    return;
                }
            },
            _ => panic!("Invalid input"),
        }
    }
    panic!("Never went downstairs");
}
