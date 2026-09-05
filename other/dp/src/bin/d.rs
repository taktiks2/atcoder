// algo: dp
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

#[fastout]
fn main() {
    input! {
        n: usize,
        w: usize,
        wv: [(usize, u64); n],
    };

    let mut dp = vec![vec![0u64; w + 1]; n + 1];

    for i in 0..n {
        let (wi, vi) = wv[i];
        for j in 0..=w {
            dp[i + 1][j] = dp[i][j];

            if j >= wi {
                dp[i + 1][j] = dp[i + 1][j].max(dp[i][j - wi] + vi);
            }
        }
    }

    println!("{}", dp[n][w]);
}

// alt: 参照するのは直前の 1 行だけなので、1 次元配列で in-place に更新できる。
//      メモリが O(NW) -> O(W)。ただし j を降順ループにする必要がある。
//      昇順だと更新済みの dp[j - wi] (= 今の品物を既に使った値) を読んでしまい、
//      同じ品物を複数回入れる = 個数無制限ナップサック (dp_f) の遷移になる
// let mut dp = vec![0u64; w + 1];
// for &(wi, vi) in &wv {
//     for j in (wi..=w).rev() {
//         dp[j] = dp[j].max(dp[j - wi] + vi);
//     }
// }
// println!("{}", dp[w]);
