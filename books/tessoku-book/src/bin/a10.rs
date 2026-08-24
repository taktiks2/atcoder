use proconio::input;
use std::cmp::max;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
        d: usize,
        lr: [(usize, usize); d]
    };

    let mut p = vec![0; n + 1];
    for i in 1..=n {
        p[i] = max(p[i - 1], a[i - 1])
    }

    let mut q = vec![0; n + 1];
    a.reverse();
    for i in 1..=n {
        q[i] = max(q[i - 1], a[i - 1])
    }
    q.reverse();

    for (l, r) in lr {
        let ans = max(p[l - 1], q[r]);
        println!("{}", ans)
    }
}
