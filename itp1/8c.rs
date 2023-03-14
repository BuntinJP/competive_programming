use std::io::Read;
fn main() {
    let mut s = String::new();
    std::io::stdin().read_to_string(&mut s).unwrap();
    s = s.trim().to_lowercase();
    let a = (b'a'..b'z' + 1).map(|c| c as char);
    let n = a.map(|x| (x, s.chars().filter(|&c| c == x).count()));
    for i in n {
        println!("{} : {}", i.0, i.1);
    }
}
