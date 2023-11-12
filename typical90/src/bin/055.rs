use proconio::input;

fn main() {
    input! {
        n: usize,
        p: u64,
        q: u64,
        a: [u64; n],
    };

    let mut ans = 0;

    for i in 0..n {
        for j in 0..i {
            for k in 0..j {
                for l in 0..k {
                    for m in 0..l {
                        if a[i] * a[j] * a[k] * a[l] * a[m] % p == q {
                            ans += 1;
                        }
                    }
                }
            }
        }
    }

    println!("{}", ans)
}
