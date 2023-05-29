use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        abcd: [(usize, usize, usize, usize, usize); n],
    }

    let mut left = 0;
    let mut right = 1_000_000_000;

    while right - left > 1 {
        let mid = (left + right) / 2;

        let mut s = HashSet::new();

        for &(a, b, c, d, e) in &abcd {
            let mut bits = 0;
            if a >= mid {
                bits |= 1 << 0;
            }
            if b >= mid {
                bits |= 1 << 1;
            }
            if c >= mid {
                bits |= 1 << 2;
            }
            if d >= mid {
                bits |= 1 << 3;
            }
            if e >= mid {
                bits |= 1 << 4;
            }
            s.insert(bits);
        }

        let mut can = false;

        for &x in &s {
            for &y in &s {
                for &z in &s {
                    if x | y | z == 31 {
                        can = true;
                    }
                }
            }
        }

        if can {
            left = mid;
        } else {
            right = mid;
        }
    }

    println!("{}", left);
}
