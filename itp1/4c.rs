fn main() {
    loop {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).ok();
        let n: Vec<&str> = line.trim().split(' ').collect();
        let a: i32 = n[0].parse().unwrap();
        let b: i32 = n[2].parse().unwrap();
        let op = n[1];
        match op {
            "+" => println!("{}", a + b),
            "-" => println!("{}", a - b),
            "*" => println!("{}", a * b),
            "/" => println!("{}", a / b),
            "?" => break,
            _ => println!("error"),
        }
    }
}
