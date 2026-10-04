// algo: simulation
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };

    let mut ans = 0usize;
    for x in (n / 2)..n {
        ans += a[x];
    }

    println!("{ans}");
}

// alt: スライスの sum で書ける — 手動ループと mut が消える (Rust らしさ)
// let ans: usize = a[n / 2..].iter().sum();
// println!("{ans}");

// alt: input! を 2 回に分けて前半を読み捨てる — 後半 N/2 個しか保持しない (メモリ半減)
// input! { n: usize };
// input! { _: [usize; n / 2], b: [usize; n / 2] };
// println!("{}", b.iter().sum::<usize>());
