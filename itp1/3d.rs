fn main() {
    let v = read_line_sepated_integers();
    let a = v[0];
    let b = v[1];
    let c = v[2];
    //count divisor of c in range [a, b]
    let mut count = 0;
    for i in a..(b + 1) {
        if c % i == 0 {
            count += 1;
        }
    }
    println!("{}", count);
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
