use std::io::*;
fn main() {
    let s = stdin().lock().lines().next().unwrap().unwrap();
    let p = stdin().lock().lines().next().unwrap().unwrap();
    println!(
        "{}",
        if (s.clone() + &s).contains(&p) {
            "Yes"
        } else {
            "No"
        }
    );
}
