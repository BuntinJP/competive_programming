use std::io::*;
fn main() {
    loop {
        let mut l = String::new();
        stdin().read_line(&mut l).unwrap();
        if l.trim() == "0" {
            break;
        }
        println!(
            "{}",
            l.trim()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .sum::<i32>()
        )
    }
}
