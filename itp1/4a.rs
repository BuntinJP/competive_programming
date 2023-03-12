fn main() {
    let v = read_line_sepated_integers();
    let a = v[0];
    let b = v[1];
    let c = a / b;
    let d = a % b;
    let a: f64 = a.into();
    let b: f64 = b.into();
    let e = a / b;
    println!("{} {} {:.5}", c, d, e);
}

fn read_line_sepated_integers() -> Vec<i32> {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    let v: Vec<i32> = s
        .trim()
        .split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect();
    v
}
