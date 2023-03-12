fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let n: i32 = line.trim().parse().unwrap();
    let h = n / 3600;
    let m = (n - h * 3600) / 60;
    let s = n - h * 3600 - m * 60;
    println!("{}:{}:{}", h, m, s);
}
