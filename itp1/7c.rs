fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let q: Vec<usize> = s
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let (r, c) = (q[0], q[1]);
    let mut a = vec![vec![0; c]; r];
    for i in 0..r {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        let q: Vec<usize> = s
            .trim()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        for j in 0..c {
            a[i][j] = q[j];
        }
    }
    let mut b = vec![vec![0; c + 1]; r + 1];
    for i in 0..r {
        for j in 0..c {
            b[i][j] = a[i][j];
        }
    }
    for i in 0..r {
        for j in 0..c {
            b[i][c] += a[i][j];
        }
    }
    for i in 0..r {
        for j in 0..c {
            b[r][j] += a[i][j];
        }
    }
    for i in 0..r {
        b[r][c] += b[i][c];
    }
    for i in 0..r + 1 {
        for j in 0..c + 1 {
            if j == c {
                print!("{}", b[i][j]);
            } else {
                print!("{} ", b[i][j]);
            }
        }
        println!("");
    }
}
