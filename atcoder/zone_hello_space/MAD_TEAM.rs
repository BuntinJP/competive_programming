use proconio::input;
fn main() {
    input! { n: usize, a: [[usize; 5]; n] }
    let mut ok = 0usize;
    let mut ng = 1_000_000_001usize;
    while ng - ok > 1 {
        let mid = (ok + ng) / 2;
        let mut flag = false;
        let mut cnt = vec![0; 32];
        for i in 0..n {
            let mut bit = 0;
            for j in 0..5 {
                if a[i][j] >= mid {
                    bit = bit | (1 << j);
                }
            }
            cnt[bit] += 1;
        }
        for i in 0..32 {
            for j in 0..32 {
                for k in 0..32 {
                    cnt[i] -= 1;
                    cnt[j] -= 1;
                    cnt[k] -= 1;
                    if cnt[i] >= 0 && cnt[j] >= 0 && cnt[k] >= 0 {
                        let bit = i | j | k;
                        if bit == 31 {
                            flag = true;
                        }
                    }
                    cnt[i] += 1;
                    cnt[j] += 1;
                    cnt[k] += 1;
                }
            }
        }
        if flag {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    println!("{}", ok);
}
