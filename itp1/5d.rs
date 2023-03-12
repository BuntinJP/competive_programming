fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    let n = s.trim().parse::<usize>().ok().unwrap();
    for i in 1..n + 1 {
        if i % 3 == 0 || i.to_string().contains("3") {
            print!(" {}", i);
        }
    }
    println!("");
}
