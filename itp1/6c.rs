fn main() {
    let mut n = String::new();
    std::io::stdin().read_line(&mut n).ok();
    let n = n.trim().parse().unwrap();
    let mut rooms = vec![vec![0 as i32; 10]; 12];
    for _ in 0..n {
        let mut moves = String::new();
        std::io::stdin().read_line(&mut moves).ok();
        let moves: Vec<i32> = moves
            .trim()
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        rooms[((moves[0] - 1) * 3 + moves[1] - 1) as usize][(moves[2] - 1) as usize] += moves[3];
    }
    for i in 0..4 {
        for j in 0..3 {
            println!(
                " {}",
                rooms[i * 3 + j]
                    .iter()
                    .map(|c| c.to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            );
        }
        if i < 3 {
            println!("{}", "####################");
        }
    }
}
