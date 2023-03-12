fn main() {
    let mut input = read_line_sepated_integers();
    let W = input[0];
    let H = input[1];
    let x = input[2];
    let y = input[3];
    let r = input[4];
    // if the surcle is in the rectanble
    if x - r >= 0 && x + r <= W && y - r >= 0 && y + r <= H {
        println!("Yes");
    } else {
        println!("No");
    }
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
