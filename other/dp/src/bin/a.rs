// algo: dp
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
use std::cmp::min;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

// #[fastout]
fn main() {
    input! {
        n: usize,
        h: [i64; n],
    };

    let mut arr = vec![0i64; n];

    for (i, &height) in h.iter().enumerate().skip(1) {
        dbg!(i, height);
        if i > 1 {
            let step1 = (h[i - 1] - height).abs() + arr[i - 1];
            let step2 = (h[i - 2] - height).abs() + arr[i - 2];
            arr[i] = if step1 > step2 { step2 } else { step1 };
            dbg!(step1, step2, arr[i]);
        } else {
            let step1 = (h[i - 1] - height).abs();
            arr[i] = step1;
            dbg!(step1, arr[i]);
        }
    }

    dbg!(&arr);

    println!("{}", arr[n - 1]);
}

// alt: min() と (1..=2) のループで「2 通りのうち小さい方」を書くと、手書きの > 比較と
//      i == 1 の分岐が両方消える。filter(|&j| j <= i) が「まだ 2 つ前が無い」場合を吸収する。
//      この形は 2 を K に変えるだけで Frog 2 (B) に一般化できる
// let mut dp = vec![0i64; n];
// for i in 1..n {
//     dp[i] = (1..=2)
//         .filter(|&j| j <= i)
//         .map(|j| dp[i - j] + (h[i - j] - h[i]).abs())
//         .min()
//         .unwrap();
// }
// println!("{}", dp[n - 1]);

// alt: 配る DP。main は「i にはどこから来られるか」を集める形 (もらう DP) だが、
//      「i からどこへ飛べるか」を配る形でも書ける。未確定を i64::MAX で初期化しておけば
//      始点の特別扱いが dp[0] = 0 の 1 行に収まり、飛び先が不規則な問題へも素直に伸びる
// let mut dp = vec![i64::MAX; n];
// dp[0] = 0;
// for i in 0..n {
//     for j in 1..=2 {
//         if i + j < n {
//             dp[i + j] = min(dp[i + j], dp[i] + (h[i] - h[i + j]).abs());
//         }
//     }
// }
// println!("{}", dp[n - 1]);

// alt: dp[i] は直前 2 つしか参照しないので、配列を持たず (2 つ前, 1 つ前) の 2 変数で回せる。
//      メモリが O(N) -> O(1)。さらに fold で畳み込めば mut も添字への代入も消える
// let ans = (2..n)
//     .fold((0i64, (h[1] - h[0]).abs()), |(prev2, prev1), i| {
//         (
//             prev1,
//             min(
//                 prev1 + (h[i - 1] - h[i]).abs(),
//                 prev2 + (h[i - 2] - h[i]).abs(),
//             ),
//         )
//     })
//     .1;
// println!("{}", ans);
