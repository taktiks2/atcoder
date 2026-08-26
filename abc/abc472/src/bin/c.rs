// よりRustらしい解法
// use std::collections::VecDeque;
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        k: i64,
        a: [i64; n],
    };

    let mut eaten = vec![0i64; n];
    let mut sum = 0i64;

    for i in 0..n {
        if i >= m {
            sum -= eaten[i - m];
        }
        if sum + a[i] <= k {
            sum += a[i];
            eaten[i] = a[i];
            println!("Yes");
        } else {
            println!("No");
        }
    }

    // よりRustらしい解法
    // let mut window: VecDeque<i64> = VecDeque::with_capacity(m);
    // let mut sum = 0i64;
    //
    // for &x in &a {
    //     if window.len() == m {
    //         sum -= window.pop_front().unwrap();
    //     }
    //     let eat = sum + x <= k;
    //     let c = if eat { x } else { 0 };
    //     sum += c;
    //     window.push_back(c);
    //     println!("{}", if eat { "Yes" } else { "No" });
    // }
}
