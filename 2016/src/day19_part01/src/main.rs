use std::env::args;

fn main() {
    let elf_count = get_elf_count();
    let answer = 2 * (elf_count - (2 as usize).pow((elf_count as f64).log2() as u32)) + 1;
    println!("Answer: {}", answer);
}

fn get_elf_count() -> usize {
    args().skip(1).next().unwrap().parse().unwrap()
}
