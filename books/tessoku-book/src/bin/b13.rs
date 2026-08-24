use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n]
    };

    let mut r = vec![0; n];

    for i in 0..n {
        let mut acc = 0;
        r[i] = i;
        while r[i] < n && acc + a[r[i]] <= k {
            acc += a[r[i]];
            r[i] += 1
        }
    }

    let mut ans = 0;
    for (i, q) in r.iter().enumerate() {
        ans += q - i
    }
    println!("{}", ans)
}
