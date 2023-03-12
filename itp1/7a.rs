fn main() {
    loop {
        //
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        if input.trim() == "-1 -1 -1" {
            break;
        }
        let score: Vec<i32> = input
            .trim()
            .split_whitespace()
            .flat_map(str::parse)
            .collect();
        let a = score[0];
        let b = score[1];
        let c = score[2];
        let usual_score = a + b;
        if a == -1 || b == -1 {
            println!("F");
            continue;
        }
        if usual_score >= 80 {
            println!("A");
            continue;
        }
        if usual_score >= 65 {
            println!("B");
            continue;
        }
        if usual_score >= 50 {
            println!("C");
            continue;
        }
        if usual_score >= 30 {
            if c >= 50 {
                println!("C");
                continue;
            } else {
                println!("D");
                continue;
            }
        }
        println!("F");
    }
}
