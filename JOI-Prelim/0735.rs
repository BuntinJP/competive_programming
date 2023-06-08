use std::io::{self, Write};
fn main() {
    let mut f = String::new();
    io::stdin().read_line(&mut f).unwrap();
    let f: i32 = f.trim().parse().unwrap();
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    let s: i32 = s.trim().parse().unwrap();
    println!("{}", f * s);
}
