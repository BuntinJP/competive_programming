fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let mut ss = String::new();
    std::io::stdin().read_line(&mut ss).unwrap();
    let v: Vec<i64> = ss.split_whitespace().map(|x| x.parse().unwrap()).collect();
    println!(
        "{} {} {}",
        v.iter().min().unwrap(),
        v.iter().max().unwrap(),
        v.iter().sum::<i64>()
    );
}
