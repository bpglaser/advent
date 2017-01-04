extern crate itertools;

use itertools::Itertools;

static INPUT: [usize; 20] = [50, 44, 11, 49, 42, 46, 18, 32, 26, 40, 21, 7, 18, 43, 10, 47, 36, 24, 22, 40];

fn main() {
    let mut min_size = usize::max_value();
    let mut min_combinations = vec![];
    for i in 1..INPUT.len() + 1 {
        for combination in INPUT.iter().combinations(i) {
            if combination.iter().cloned().sum::<usize>() == 150 {
                let comb_len = combination.len();
                
                if comb_len > min_size {
                    continue;
                }

                if comb_len < min_size {
                    min_size = comb_len;
                    min_combinations.clear();
                }

                min_combinations.push(combination);
            }
        }
    }

    println!("Answer: {}", min_combinations.len());
}
