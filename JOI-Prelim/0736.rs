use std::io::{self};
fn main() {
    let mut f = String::new();
    io::stdin().read_line(&mut f).unwrap();
    let f: i32 = f.trim().parse().unwrap();
    let mejiro = (f / 10) == (f % 10);
    if mejiro {
        println!("1");
    } else {
        println!("0");
    }
}
