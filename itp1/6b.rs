fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let n: i32 = s.trim().parse().unwrap();
    let mut spade: Vec<i32> = Vec::new();
    let mut heart: Vec<i32> = Vec::new();
    let mut club: Vec<i32> = Vec::new();
    let mut diamond: Vec<i32> = Vec::new();
    for _ in 0..n {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        let mut iter = s.split_whitespace();
        let mark = iter.next().unwrap();
        let num: i32 = iter.next().unwrap().parse().unwrap();
        match mark {
            "S" => spade.push(num),
            "H" => heart.push(num),
            "C" => club.push(num),
            "D" => diamond.push(num),
            _ => (),
        }
    }
    for i in 1..14 {
        if !spade.contains(&i) {
            println!("S {}", i);
        }
    }
    for i in 1..14 {
        if !heart.contains(&i) {
            println!("H {}", i);
        }
    }
    for i in 1..14 {
        if !club.contains(&i) {
            println!("C {}", i);
        }
    }
    for i in 1..14 {
        if !diamond.contains(&i) {
            println!("D {}", i);
        }
    }
}
