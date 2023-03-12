fn main() {
    let mut input = read_line_sepated_integers();
    input.sort();
    println!("{} {} {}", input[0], input[1], input[2]);
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
