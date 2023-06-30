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

fn main() {
    let x = read_line_sepated_integers()[1];
    let mut a = read_line_sepated_integers();
    a.sort();
    for i in 0..a.len() {
        let p = a[i] - x;
        let n = a[i] + x;
        if a.binary_search(&p).is_ok() || a.binary_search(&n).is_ok() {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
