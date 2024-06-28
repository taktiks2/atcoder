use proconio::input;

fn main() {
    input! {
        n: usize,
        k: i64,
        a: [i64; n]
    };

    let mut r = vec![0; n];
    // しゃくとり法
    for i in 0..n {
        // スタート地点決め
        if i > 0 {
            r[i] = r[i - 1]
        }

        while r[i] < n - 1 && a[r[i] + 1] - a[i] <= k {
            r[i] += 1
        }
    }

    let mut ans = 0;
    for (i, q) in r.iter().enumerate() {
        ans += q - i
    }

    println!("{}", ans)
}
