use std::env::args;
use std::ptr::copy;

fn main() {
    let input: usize = args().nth(1).unwrap().parse().unwrap();

    let mut buf = [0u32; 2018];
    let mut len = 1;
    let mut pos = 0;

    for i in 1..2018 {
        #[cfg(feature = "debug_print")]
        print_buf(&buf, pos, len);

        let n = ((pos + input) % len) + 1;
        unsafe {
            shift_right(&mut buf, n, len);
        }
        buf[n] = i;
        pos = n;
        len += 1;
    }

    #[cfg(feature = "debug_print")]
    println!("{:?}", buf.iter().skip(pos - 3).take(7).collect::<Vec<_>>());

    println!("{}", buf[pos + 1]);
}

unsafe fn shift_right<T>(buf: &mut [T], i: usize, mut len: usize) {
    if i + 1 + len >= buf.len() {
        len = buf.len() - (i + 1);
    }
    let left = buf.as_ptr().offset(i as isize);
    let right = buf.as_mut_ptr().offset((i + 1) as isize);
    copy(left, right, len);
}

#[cfg(feature = "debug_print")]
fn print_buf(buf: &[u32], pos: usize, len: usize) {
    print!("> ");
    for (n, val) in buf.iter().take(len).enumerate() {
        if n == pos {
            print!("({}) ", val);
        } else {
            print!("{} ", val);
        }
    }
    println!();
}