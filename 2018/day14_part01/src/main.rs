use std::env::args;

fn main() {
    let input = args().nth(1).unwrap().parse().unwrap();
    println!("{:010}", solve(input));
}

fn solve(input: usize) -> u64 {
    let mut buf = vec![3, 7];
    let mut i = 0;
    let mut j = 1;

    while buf.len() < input + 10 {
        let sum = buf[i] + buf[j];
        if sum >= 10 {
            buf.push(sum / 10);
            buf.push(sum % 10);
        } else {
            buf.push(sum);
        }
        i = (i + 1 + buf[i] as usize) % buf.len();
        j = (j + 1 + buf[j] as usize) % buf.len();
    }

    build_ans(&buf[input..input + 10])
}

fn build_ans(buf: &[u8]) -> u64 {
    assert!(buf.len() == 10);
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
    fn given() {
        assert_eq!(solve(9), 5158916779);
        assert_eq!(solve(5), 0124515891);
        assert_eq!(solve(18), 9251071085);
        assert_eq!(solve(2018), 5941429882);
    }
}
