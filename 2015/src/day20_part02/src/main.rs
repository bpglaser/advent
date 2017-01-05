use std::env::args;

fn main() {
    let target = get_input();

    let mut i = 1;
    loop {
        if divisors(i).iter().filter(|n| i / *n <= 50).sum::<usize>() * 11 >= target {
            break;
        }
        i += 1;
    }
    println!("Answer: {}", i);
}

fn divisors(n: usize) -> Vec<usize> {
    let mut result = vec![];
    let sqrt = (n as f64).sqrt() as usize + 1;
    for i in 1..sqrt {
        if n % i == 0 {
            result.push(i);
            let compliment = n / i;
            if compliment != i {
                result.push(compliment);
            }
        }
    }
    result
}

fn get_input() -> usize {
    args().nth(1).expect("Invalid number of args").parse().expect("Arg not an int")
}
