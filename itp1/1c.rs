fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .ok()
        .expect("Failed to read line");
    let x: Vec<i32> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    println!("{} {}", x[0] * x[1], 2 * x[0] + 2 * x[1])
}
