fn main() {
    let mut s = String::new();
    let mut n = String::new();
    std::io::stdin().read_line(&mut n).ok();
    std::io::stdin().read_line(&mut s).ok();
    let n: i32 = n.trim().parse().unwrap();
    let mut v: Vec<i32> = s.split_whitespace().map(|x| x.parse().unwrap()).collect();
    for i in (0..n).rev() {
        print!("{}", v[i as usize]);
        if i != 0 {
            print!(" ");
        }
    }
    println!("");
}
