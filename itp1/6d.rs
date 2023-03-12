use std::vec;

fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let n: Vec<i32> = s
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    // n = [a,b]
    let mut queue = vec![vec![0; n[1] as usize]; n[0] as usize];
    for i in 0..n[0] {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        let v: Vec<i32> = s
            .trim()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        for j in 0..n[1] {
            queue[i as usize][j as usize] = v[j as usize];
        }
    }
    let mut b = vec![0; n[1] as usize];
    for i in 0..n[1] {
        let mut inp = String::new();
        std::io::stdin().read_line(&mut inp).unwrap();
        let num: i32 = inp.trim().parse().unwrap();
        b[i as usize] = num;
    }
    let mut ans = vec![0; n[0] as usize];
    for i in 0..n[0] {
        let mut el = 0;
        for j in 0..n[1] {
            el += queue[i as usize][j as usize] * b[j as usize];
        }
        ans[i as usize] = el;
    }
    for i in 0..n[0] {
        println!("{}", ans[i as usize]);
    }
}
