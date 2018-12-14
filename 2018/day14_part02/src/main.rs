use std::env::args;

fn main() {
    let raw_input = args().nth(1).unwrap();
    let input = raw_input.parse().unwrap();
    println!("{}", solve(input, raw_input.len()));
}

fn solve(input: u64, num_chars: usize) -> u64 {
    let mut buf = vec![3u8, 7u8];
    let mut i = 0;
    let mut j = 1;

    loop {
        let sum = buf[i] + buf[j];
        if sum >= 10 {
            buf.push(sum / 10);
            buf.push(sum % 10);
        } else {
            buf.push(sum);
        }

        if buf.len() >= num_chars && build_ans(&buf[buf.len() - num_chars..]) == input {
            return (buf.len() - num_chars - 1) as u64;
        } else if buf.len() > num_chars
            && build_ans(&buf[buf.len() - num_chars - 1..buf.len() - 1]) == input
        {
            return (buf.len() - num_chars - 1) as u64;
        }

        i = (i + 1 + buf[i] as usize) % buf.len();
        j = (j + 1 + buf[j] as usize) % buf.len();
    }
}

fn build_ans(buf: &[u8]) -> u64 {
    let mut sum = 0;
    for n in buf {
        sum *= 10;
        sum += *n as u64;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_given() {
        assert_eq!(solve(51589, 5), 9);
        assert_eq!(solve(01245, 5), 5);
        assert_eq!(solve(92510, 5), 18);
        assert_eq!(solve(59414, 5), 2018);
    }

    #[test]
    fn test_build_ans() {
        assert_eq!(build_ans(&[1, 2, 3]), 123);
        assert_eq!(build_ans(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]), 0123456789);
    }
}
