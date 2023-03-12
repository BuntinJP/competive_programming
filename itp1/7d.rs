fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    //input n,m,l
    let mut iter = s.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();
    let l: usize = iter.next().unwrap().parse().unwrap();
    //input matrix A
    let mut a: Vec<Vec<i64>> = vec![vec![0; m]; n];
    for i in 0..n {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        let mut iter = s.split_whitespace();
        for j in 0..m {
            a[i][j] = iter.next().unwrap().parse().unwrap();
        }
    }
    //input matrix B
    let mut b = vec![vec![0; l]; m];
    for i in 0..m {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        let mut iter = s.split_whitespace();
        for j in 0..l {
            b[i][j] = iter.next().unwrap().parse().unwrap();
        }
    }
    //culc matrix C
    let mut c = vec![vec![0; l]; n];
    for i in 0..n {
        for j in 0..l {
            for k in 0..m {
                c[i][j] += a[i][k] * b[k][j];
            }
        }
    }
    //output matrix C
    for i in 0..n {
        for j in 0..l {
            if j != 0 {
                print!(" ");
            }
            print!("{}", c[i][j]);
        }
        println!("");
    }
}
