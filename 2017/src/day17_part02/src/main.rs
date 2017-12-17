use std::env::args;

fn main() {
    let input: usize = args().nth(1).unwrap().parse().unwrap();

    let mut len = 1;
    let mut pos = 0;
    let mut ans = 0;

    for i in 0..50_000_000 {
        let n = ((pos + input) % len) + 1;
        if n == 1 {
            ans = i + 1;
        }
        pos = n;
        len += 1;
    }

    println!("{}", ans);
}