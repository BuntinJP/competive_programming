fn main() {
    let mut x = 1;
    loop {
        let v = read_line_sepated_integers();
        if v[0] == 0 {
            break;
        };
        println!("Case {}: {}", x, v[0]);
        x += 1;
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
