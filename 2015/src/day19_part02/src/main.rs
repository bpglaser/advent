static INPUT_INSTRUCTIONS: &'static str = include_str!("instructions.txt");
static INPUT_MOLECULE: &'static str = include_str!("molecule.txt");

fn main() {
    let mut instructions = vec![];
    for line in INPUT_INSTRUCTIONS.trim().lines() {
        let split: Vec<_> = line.split(" => ").collect();
        instructions.push((split[0], split[1]));
    }
    instructions.sort_by(|a, b| b.1.len().cmp(&a.1.len())); // Sort biggest to smallest value

    let mut working_molecule = INPUT_MOLECULE.trim().to_owned();
    println!("[0] => {}", working_molecule);
    let mut count = 0;

    'outer: loop {
        for &(key, value) in instructions.iter() {
            let num_replaced = custom_replace(&mut working_molecule, value, key);
            if num_replaced > 0 {
                count += num_replaced;
                println!("[{}] => {}", count, working_molecule);
                if working_molecule == "e" {
                    break 'outer;
                }
            }
        }
    }

    println!("Answer: {}", count);
}

fn custom_replace(input: &mut String, from: &str, to: &str) -> usize {
    let result = input.matches(from).count();
    *input = input.replace(from, to);
    result
}
