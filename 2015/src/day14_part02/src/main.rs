use std::collections::HashMap;
use std::env::args;
use std::fs::File;
use std::io::Read;

fn main() {
    let (target_duration, path) = get_args();
    let all_reindeer: Vec<Reindeer> = load_lines(&path).iter().map(|s| Reindeer::parse(s)).collect();
    let mut scoreboard = HashMap::new();

    for i in 1..target_duration + 1 {
        let snapshot_winners = get_snapshot_winners(i, &all_reindeer);
        for winner in snapshot_winners {
            let mut score = scoreboard.entry(winner).or_insert(0);
            *score += 1;
        }
    }

    let winner = scoreboard.iter().max_by_key(|&(_, v)| v).expect("Failed to find winner");
    println!("Winner: {} => {}", winner.0, winner.1);
}

fn get_snapshot_winners(target_duration: usize, all_reindeer: &[Reindeer]) -> Vec<&str> {
    let traveled_distances = find_distances(target_duration, all_reindeer);
    let max = traveled_distances.iter().max_by_key(|&(_, v)| v).expect("Failed to find max").1;
    let mut result = vec![];
    for (key, value) in traveled_distances.iter() {
        if value == max {
            result.push(*key);
        }
    }
    result
}

fn find_distances(target_duration: usize, all_reindeer: &[Reindeer]) -> HashMap<&str, usize> {
    let mut traveled_distances: HashMap<&str, usize> = HashMap::new();

    for reindeer in all_reindeer.iter() {
        let total_duration = reindeer.fly_duration + reindeer.rest_duration;

        let mut distance = traveled_distances.entry(&reindeer.name).or_insert(0);
        let cycle_count = target_duration / total_duration;
        *distance += cycle_count * reindeer.speed * reindeer.fly_duration;

        let remaining = target_duration % total_duration;
        if remaining >= reindeer.fly_duration {
            *distance += reindeer.speed * reindeer.fly_duration;
        } else {
            *distance += reindeer.speed * remaining;
        }
    }

    traveled_distances
}

#[derive(Debug)]
struct Reindeer {
    name: String,
    speed: usize,
    fly_duration: usize,
    rest_duration: usize,
}

impl Reindeer {
    fn parse(s: &str) -> Reindeer {
        let words: Vec<&str> = s.split_whitespace().collect();
        Reindeer {
            name: words[0].to_owned(),
            speed: words[3].parse().unwrap(),
            fly_duration: words[6].parse().unwrap(),
            rest_duration: words[13].parse().unwrap(),
        }
    }
}

fn get_args() -> (usize, String) {
    (
        args().nth(1).expect("Invalid args").parse().expect("Invalid args"),
        args().nth(2).expect("Invalid args")
    )
}

fn load_lines(path: &str) -> Vec<String> {
    let mut file = File::open(path).expect("Error opening file");
    let mut buf = String::new();
    file.read_to_string(&mut buf).expect("Error reading file");
    buf.lines().map(|s| s.trim().to_owned()).collect()
}

#[cfg(test)]
mod test {
    #[test]
    fn given_test() {
        let all_reindeer = vec![
            ::Reindeer { name: "Comet".to_owned(), speed: 14, fly_duration: 10, rest_duration: 127 },
            ::Reindeer { name: "Dancer".to_owned(), speed: 16, fly_duration: 11, rest_duration: 162 },
        ];

        let traveled_distances = ::find_distances(1, &all_reindeer);
        assert_eq!(&14, traveled_distances.get("Comet").unwrap());
        assert_eq!(&16, traveled_distances.get("Dancer").unwrap());

        let traveled_distances = ::find_distances(10, &all_reindeer);
        assert_eq!(&140, traveled_distances.get("Comet").unwrap());
        assert_eq!(&160, traveled_distances.get("Dancer").unwrap());

        let traveled_distances = ::find_distances(11, &all_reindeer);
        assert_eq!(&140, traveled_distances.get("Comet").unwrap());
        assert_eq!(&176, traveled_distances.get("Dancer").unwrap());

        let traveled_distances = ::find_distances(137, &all_reindeer);
        assert_eq!(&140, traveled_distances.get("Comet").unwrap());
        assert_eq!(&176, traveled_distances.get("Dancer").unwrap());

        let traveled_distances = ::find_distances(1000, &all_reindeer);
        assert_eq!(&1120, traveled_distances.get("Comet").unwrap());
        assert_eq!(&1056, traveled_distances.get("Dancer").unwrap());
    }
}
