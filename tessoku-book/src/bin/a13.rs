use proconio::input;

fn main() {
    input! {
        n: usize,
        k: i64,
    };

    let mut a: Vec<i64> = vec![0; n + 9];
    for i in 1..n {
        input! {
            x: i64,
        };
        a[i] = x
    }

    let mut r = vec![0; n + 9];
    // しゃくとり法
    for i in 1..n {
        // スタート地点決め
        if i == 1 {
            r[i] = 1
        } else {
            r[i] = r[i - 1]
        }

        while r[i] < n && a[r[i] + 1] - a[i] <= k {
            r[i] += 1
        }
    }

    let mut ans = 0;
    for i in 1..n {
        ans += r[i] - i
    }

    println!("{}", ans)
}
