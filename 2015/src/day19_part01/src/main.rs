use std::collections::HashSet;

static INPUT_INSTRUCTIONS: &'static str = include_str!("instructions.txt");
static INPUT_MOLECULE: &'static str = include_str!("molecule.txt");

fn main() {
    let mut instructions = vec![];
    for line in INPUT_INSTRUCTIONS.trim().lines() {
        let split: Vec<_> = line.split(" => ").collect();
        instructions.push((split[0], split[1]));
    }
    let molecule: Vec<char> = INPUT_MOLECULE.chars().collect();

    let mut unique_molecules = HashSet::new();

    for &(key, value) in instructions.iter() {
        for (i, replacement) in INPUT_MOLECULE.match_indices(key) {
            let (before, substring) = molecule.split_at(i);

            let mut new_molecule: String = before.iter().cloned().collect();
            new_molecule.push_str(value);

            let (_, after) = substring.split_at(replacement.len());

            for c in after.iter().cloned() {
                new_molecule.push(c);
            }

            unique_molecules.insert(new_molecule);
        }
    }

    println!("Answer: {}", unique_molecules.len());
}
