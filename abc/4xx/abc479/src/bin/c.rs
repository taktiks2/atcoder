// algo: brute-force
// solve: self 2026-10-12
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

// 体積の合計 W 以下でおいしさの合計を最大化する。0-1 ナップサックだが、W, B_i <= 10^9
// なので「体積」を添字にする DP は配列が 10^9 要素になって組めない。A_i も 10^9 までなので
// 「おいしさ」を添字にする DP も無理。
//
// 代わりに N <= 20 が小さいので、選び方 2^N 通りを bit 全探索する。bit の i ビット目が
// 1 なら i 個目を選ぶ。2^20 ≈ 10^6 通り × N = 2×10^7 回の加算で十分間に合う。
//
// 型: 合計は最大 20 × 10^9 = 2×10^10 で、u32 の上限 4.3×10^9 も超える。usize (64 bit) が要る。
//
// bit >> i & 1 == 1 は、Rust では ((bit >> i) & 1) == 1 と読まれる (& が == より強い)。
// C/C++ では == の方が強く bit >> i & (1 == 1) になってしまうので括弧が要るが、Rust では
// 括弧無しで正しい。
//
// 何も選ばない bit = 0 も cube = 0 <= W で候補に入り、ans = 0 になる。全部詰められない
// サンプル 2 の答え 0 はこれで出る。
// 出力は 1 行なので #[fastout] は本問では効いていない。

#[fastout]
fn main() {
    input! {
        n: usize,
        w: usize,
        ab: [(usize, usize); n],
    };

    let mut ans = 0usize;
    for bit in 0..(1 << n) {
        let mut cube = 0usize;
        let mut deli = 0usize;
        for i in 0..n {
            let (ai, bi) = ab[i];
            if bit >> i & 1 == 1 {
                deli += ai;
                cube += bi;
            }
        }

        if cube <= w {
            ans = ans.max(deli);
        }
    }

    println!("{ans}");
}

// alt: 構造はそのまま。削ったのは:
//      - for i in 0..n + let (ai, bi) = ab[i] → enumerate でタプルを直接受ける
//      - 0usize の型注釈 3 か所
// let mut ans = 0;
// for bit in 0..(1 << n) {
//     let mut cube = 0;
//     let mut deli = 0;
//     for (i, &(ai, bi)) in ab.iter().enumerate() {
//         if bit >> i & 1 == 1 {
//             deli += ai;
//             cube += bi;
//         }
//     }
//     if cube <= w {
//         ans = ans.max(deli);
//     }
// }

// alt: itertools の powerset で部分集合を直接列挙する。ビット演算が消えて「全部分集合の
//      うち体積 W 以下のものの最大」が宣言的に書ける。部分集合ごとに Vec を確保するが、
//      N = 20 でも手元で 0.02 秒程度。空集合も含まれるので unwrap は安全
// let ans = ab
//     .iter()
//     .powerset()
//     .filter(|set| set.iter().map(|&&(_, b)| b).sum::<usize>() <= w)
//     .map(|set| set.iter().map(|&&(a, _)| a).sum::<usize>())
//     .max()
//     .unwrap();
