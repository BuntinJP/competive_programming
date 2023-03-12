fn solve(i: i32, j: i32, a: i32, b: i32) -> bool {
    if i == 0 || i == (a - 1) || j == 0 || j == (b - 1) {
        return true;
    }
    return false;
}

fn main() {
    loop {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let mut iter = line.split_whitespace();
        let a: i32 = iter.next().unwrap().parse().unwrap();
        let b: i32 = iter.next().unwrap().parse().unwrap();
        if a == 0 && b == 0 {
            break;
        }
        let mut out = String::new();
        for i in 0..a {
            for j in 0..b {
                if solve(i, j, a, b) {
                    out.push_str("#");
                } else {
                    out.push_str(".");
                }
            }
            out.push_str("\n");
        }
        println!("{}", out);
    }
}
