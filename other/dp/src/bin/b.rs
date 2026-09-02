// algo: dp
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        k: usize,
        h: [i64; n],
    };

    let mut dp = vec![0i64; n];
    for i in 1..n {
        dp[i] = (1..=k)
            .filter(|&j| j <= i)
            .map(|j| dp[i - j] + (h[i - j] - h[i]).abs())
            .min()
            .unwrap();
    }

    let ans = dp[n - 1];
    println!("{ans}");
}

// alt: 窓の範囲そのものを Range で書く。filter(|&j| j <= i) という「はみ出しを捨てる」
//      後始末が消え、saturating_sub が i < K の端を吸収する。添字も i - j ではなく
//      j 自身になり、「直前 K 個の dp から選ぶ」という漸化式の形がそのまま読める
// let mut dp = vec![0i64; n];
// for i in 1..n {
//     dp[i] = (i.saturating_sub(k)..i)
//         .map(|j| dp[j] + (h[j] - h[i]).abs())
//         .min()
//         .unwrap();
// }
// println!("{}", dp[n - 1]);

// alt: 配る DP。main は「i にはどこから来られるか」を集める形 (もらう DP) だが、
//      「i から先 K 個へ配る」形でも書ける。未確定を i64::MAX で初期化しておけば
//      始点の特別扱いが dp[0] = 0 の 1 行に収まり、飛び先が不規則な問題へも素直に伸びる
// let mut dp = vec![i64::MAX; n];
// dp[0] = 0;
// for i in 0..n {
//     for j in 1..=k {
//         if i + j < n {
//             dp[i + j] = dp[i + j].min(dp[i] + (h[i] - h[i + j]).abs());
//         }
//     }
// }
// println!("{}", dp[n - 1]);

// alt: 参照するのは常に直前 K 個だけなので、全体の配列を持たず (dp, h) の組を長さ K の
//      VecDeque で持ち回せる。メモリが O(N) -> O(K)。「窓」がデータ構造として現れる形。
//      なお、コストが |h[j] - h[i]| で i にも依存するため、単調デックによる区間最小値の
//      使い回し (O(N) 化) はここでは効かず、窓内 K 個の走査は避けられない
// let mut win: VecDeque<(i64, i64)> = VecDeque::from([(0, h[0])]);
// for &hi in h.iter().skip(1) {
//     let cur = win.iter().map(|&(d, hj)| d + (hj - hi).abs()).min().unwrap();
//     if win.len() == k {
//         win.pop_front();
//     }
//     win.push_back((cur, hi));
// }
// println!("{}", win.back().unwrap().0);
