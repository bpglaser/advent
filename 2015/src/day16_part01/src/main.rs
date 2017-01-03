use std::collections::HashMap;

static TICKER_TAPE: &'static str = include_str!("ticker_tape.txt");
static INPUT: &'static str = include_str!("input.txt");

fn main() {
    let ticker = parse_map(TICKER_TAPE);
    let aunts = parse_aunts(INPUT); 

    for (name, stats) in aunts.iter() {
        if check_match(&ticker, stats) {
            println!("Aunt found: {}", name);
            return;
        }
    }
    println!("Unable to find aunt");
}

fn check_match(ticker: &HashMap<&str, usize>, stats: &HashMap<&str, usize>) -> bool {
    for (key, value) in stats.iter() {
        if let Some(ticker_value) = ticker.get(key) {
            if value != ticker_value {
                return false;
            }
        } else {
            return false; 
        }
    }
    true
}

fn parse_map(s: &str) -> HashMap<&str, usize> {
    let mut map = HashMap::new();

    for pair in s.trim().split(|c| c == '\n' || c == ',') {
        let words: Vec<&str> = pair.split(':').map(|s| s.trim()).collect();
        map.insert(words[0], words[1].parse().expect("Unable to parse value"));
    }

    map
}

fn parse_aunts(s: &str) -> HashMap<&str, HashMap<&str, usize>> {
    let mut aunts = HashMap::new();

    for line in s.lines() {
        let first_colon_index = line.find(':').unwrap();
        let (name, stats) = line.split_at(first_colon_index);
        let stats = stats.trim_left_matches(": ");
        let submap = parse_map(stats);
        aunts.insert(name, submap);
    }

    aunts
}
