use proconio::input;

fn main() {
    input! {
        n: usize,
        s: usize,
        m: usize,
        l: usize,
    };

    let mut ans = 1_000_000_000;
    for i in 0..30 {
        for j in 0..30 {
            for k in 0..30 {
                if i * 6 + j * 8 + k * 12 >= n {
                    ans = ans.min(i * s + j * m + k * l);
                }
            }
        }
    }

    println!("{}", ans)
}
