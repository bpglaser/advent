use std::env::args;

const FACTOR_A: u64 = 16807;
const PICKY_A: u64 = 4;

const FACTOR_B: u64 = 48271;
const PICKY_B: u64 = 8;

const DIVISOR: u64 = 2147483647;

const DISCRIMINANT: u64 = 0b1111111111111111;

fn main() {
    let a: u64 = args().nth(1).unwrap().parse().unwrap();
    let b: u64 = args().nth(2).unwrap().parse().unwrap();
    let generator_a = Generator::new(FACTOR_A, PICKY_A, a);
    let generator_b = Generator::new(FACTOR_B, PICKY_B, b);

    let mut count = 0;
    for (a, b) in generator_a.zip(generator_b).take(5_000_000) {
        if a & DISCRIMINANT == b & DISCRIMINANT {
            count += 1;
        }
    }
    println!("count: {}", count);
}

struct Generator {
    factor: u64,
    picky_value: u64,
    previous_value: u64,
}

impl Generator {
    fn new(factor: u64, picky_value: u64, start: u64) -> Self {
        Generator { factor, picky_value, previous_value: start }
    }
}

impl Iterator for Generator {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let new_value = (self.previous_value * self.factor) % DIVISOR;
        self.previous_value = new_value;
        if new_value % self.picky_value == 0 {
            Some(new_value)
        } else {
            self.next()
        }
    }
}