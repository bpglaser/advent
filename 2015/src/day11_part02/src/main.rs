use std::env::args;

static VERBOTEN_CHARS: [char; 3] = ['i', 'o', 'l'];
static LETTERS: [char; 26] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm',
                              'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];

fn main() {
    let mut password: Vec<char> = get_input().chars().collect();
    loop {
        increment_password(&mut password);
        if is_valid(&password) {
            break;
        }
    }
    let password: String = password.into_iter().collect();
    println!("Next password: {}", password);
}

fn is_valid(password: &[char]) -> bool {
    !contains_verboten(password) && contains_straight_triplet(password) && contains_pairs(password)
}

fn contains_verboten(password: &[char]) -> bool {
    password.iter().any(|c| VERBOTEN_CHARS.contains(&c))
}

fn contains_straight_triplet(password: &[char]) -> bool {
    for triplet in LETTERS.windows(3) {
        for window in password.windows(3) {
            if triplet == window {
                return true;
            }
        }
    }
    false
}

fn contains_pairs(password: &[char]) -> bool {
    let mut encountered_pairs = vec![];
    let mut skip = false;
    for pair in password.windows(2) {
        if skip {
            skip = false;
            continue;
        }

        let a = pair[0];
        let b = pair[1];

        if a == b {
            if encountered_pairs.len() > 0 && !encountered_pairs.contains(&a) {
                return true;
            }
            encountered_pairs.push(a);
            skip = true;
        }
    }

    false
}

fn increment_password(password: &mut [char]) {
    let mut index = password.len() - 1;
    loop {
        {
            let c = password.get_mut(index).unwrap();

            let mut letter_index = None;
            for (n, letter) in LETTERS.iter().enumerate() {
                if letter == c {
                    letter_index = Some(n);
                }
            }
            let mut letter_index = letter_index.expect("Unable to find index for letter");

            letter_index += 1;
            if letter_index >= LETTERS.len() {
                letter_index = 0;
                *c = LETTERS[letter_index];
            } else {
                *c = LETTERS[letter_index];
                break;
            }
        }

        if index == 0 {
            for c in password.iter_mut() {
                *c = 'a';
            }
            return;
        } else {
            index -= 1;
        }
    }
}

fn get_input() -> String {
    args().nth(1).expect("Invalid args")
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_contains_straight_triplet() {
        assert!(::contains_straight_triplet(&helper("fooabcbar")));
        assert!(::contains_straight_triplet(&helper("fooxyzbar")));
        assert!(!::contains_straight_triplet(&helper("foobar")));
    }

    #[test]
    fn test_contains_pairs() {
        assert!(!::contains_pairs(&helper("aaa")));
        assert!(::contains_pairs(&helper("aabb")));
        assert!(!::contains_pairs(&helper("acabb")));
    }

    #[test]
    fn test_increment_password() {
        let mut password: Vec<char> = helper("xx");

        ::increment_password(&mut password);
        assert_eq!(helper("xy"), password);

        ::increment_password(&mut password);
        assert_eq!(helper("xz"), password);

        ::increment_password(&mut password);
        assert_eq!(helper("ya"), password);

        ::increment_password(&mut password);
        assert_eq!(helper("yb"), password);
    }

    fn helper(s: &str) -> Vec<char> {
        s.chars().collect()
    }
}
