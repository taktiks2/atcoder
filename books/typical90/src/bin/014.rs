use proconio::input;

// 貪欲法;

fn main() {
    input! {
        n: usize,
        mut a: [i64; n],
        mut b: [i64; n],
    };

    a.sort();
    b.sort();

    let mut ans = 0;

    for i in 0..n {
        ans += (a[i] - b[i]).abs();
    }

    println!("{}", ans);
}
