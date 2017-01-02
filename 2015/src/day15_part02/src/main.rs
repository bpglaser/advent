use std::env::args;
use std::fs::File;
use std::io::Read;

const PROPERY_COUNT: usize = 5;

fn main() {
    let mut names = vec![];
    let mut ingredients = vec![];

    for line in load_lines() {
        let(name, ingredient) = parse_line(&line);
        names.push(name);
        ingredients.push(ingredient);
    }

    let mut best_score = isize::min_value();

    for i in 0..101 {
        for j in 0..101 - i {
            for k in 0..101 - i - j {
                let l = 100 - i - j - k;
                let mut calorie_score = 0;
                let new_score = score(&ingredients, &[i, j, k, l], &mut calorie_score);
                if calorie_score != 500 {
                    continue;
                }
                if new_score > best_score {
                    best_score = new_score;
                }
            }
        }
    }

    println!("Best score: {}", best_score);
}

fn score(ingredients: &[[isize; PROPERY_COUNT]], state: &[usize], calorie_score: &mut isize) -> isize {
    let mut score = 1;
    for property_index in 0..PROPERY_COUNT {
        let mut property_score = 0;
        for (ingredient_index, ingredient) in ingredients.iter().enumerate() {
            property_score += ingredient[property_index] * state[ingredient_index] as isize;
        }
        if property_index == 4 {
            *calorie_score = property_score;
            continue;
        }
        if property_score <= 0 {
            return 0;
        }
        score *= property_score;
    }
    score
}

fn parse_line(s: &str) -> (String, [isize; PROPERY_COUNT]) {
    let words: Vec<&str> = s.split_whitespace().collect();
    let name = words[0].trim_right_matches(":").to_owned();
    let capacity = words[2].trim_right_matches(",").parse().unwrap();
    let durability = words[4].trim_right_matches(",").parse().unwrap();
    let flavor = words[6].trim_right_matches(",").parse().unwrap();
    let texture = words[8].trim_right_matches(",").parse().unwrap();
    let calories = words[10].parse().unwrap();
    (name, [capacity, durability, flavor, texture, calories])
}

fn load_lines() -> Vec<String> {
    let mut file = File::open(args().nth(1).expect("Invalid args")).expect("Error opening file");
    let mut buf = String::new();
    file.read_to_string(&mut buf).expect("Error reading file");
    buf.lines().map(|s| s.trim().to_owned()).collect()
}

#[cfg(test)]
mod tests {
    #[test]
    fn given_example() {
        let ingredients = [
            [-1, -2, 6, 3/*, 8*/],
            [2, 3, -2, -1/*, 3*/],
        ];
        let state = [44, 56];
        let answer = ::score(&ingredients, &state);
        assert_eq!(62842880, answer);
    }
}
