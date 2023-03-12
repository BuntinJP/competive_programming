use std::f64::consts::PI;
fn main() {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).ok();
    let r: f64 = line.trim().parse::<f64>().unwrap();
    println!("{} {}", PI * r * r, 2.0 * PI * r);
}
