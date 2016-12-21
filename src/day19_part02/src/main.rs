use std::env::args;

fn main() {
    let elf_count = get_elf_count();

    let mut the_highlander = 1;

    let mut i = 1;
    while i < elf_count {
        the_highlander = (the_highlander % i) + 1;

        if the_highlander > (i + 1) / 2 {
            the_highlander += 1;
        }
        
        i += 1;
    }

    println!("answer: {}", the_highlander);
}

fn get_elf_count() -> usize {
    args().skip(1).next().unwrap().parse().unwrap()
}
