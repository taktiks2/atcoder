// algo: dp
// solve: copy 2026-10-28
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

// x_0..x_N の割り当て 2^(N+1) 通りのうち、左から AND / OR で畳んだ結果 y_N が True に
// なるものを数える。N <= 60 なので 2^61 ≈ 2.3*10^18 通りの全探索は論外。
//
// y_i は y_{i-1} と x_i だけで決まり、それより前の x の並びは y_{i-1} の値を通してしか
// 効かない。だから i 段目までの割り当ては「y_i = True の個数 t」と「False の個数 f」の
// 2 状態に畳める。x_i の 2 通りを足し合わせると遷移は
//   AND: True になるのは (y, x) = (T, T) だけ → t' = t,      f' = 2f + t
//   OR : False になるのは (F, F) だけ         → t' = 2t + f, f' = f
// 初期状態は x_0 の 2 通りで (t, f) = (1, 1)。(1, 0) から始めるとサンプル 1 が 5 ではなく
// 3 になる。変数が N 個ではなく N+1 個あるのを見落とすと、ここでずれる。
//
// 型: t + f は毎段ちょうど 2 倍になり最大 2^61。更新途中の 2t, 2f も 2^62 以下なので u64 で
// 溢れない (i64 でも足りる)。計算量は O(N)。

#[fastout]
fn main() {
    input! {
        n: usize,
        s: [String; n],
    };

    let (mut t, mut f) = (1u64, 1u64);
    for op in &s {
        (t, f) = if op == "AND" {
            (t, 2 * f + t)
        } else {
            (2 * t + f, f)
        }
    }

    println!("{t}");
}

// alt: リファクタリング版なし — 2 状態を 1 式で同時更新するだけで、削れる冗長がない

// alt: 末尾の段から見ると DP が要らず、2 のべき乗の和 1 行で書ける (短さ・状態を持たない)
//      S_N = OR のとき: x_N = True なら x_0..x_{N-1} が何でも True → 2^N 通り。
//                       x_N = False なら y_N = y_{N-1} で、N-1 段の問題に戻る。
//      S_N = AND のとき: x_N = False は 0 通り、x_N = True なら N-1 段の問題に戻る。
//      よって ans(N) = [S_N = OR] * 2^N + ans(N-1)、ans(0) = 1 (x_0 = True)。
//      最大は全部 OR の 2^61 - 1 で u64 に収まる
// let ans: u64 = 1 + (1..=n).filter(|&i| s[i - 1] == "OR").map(|i| 1u64 << i).sum::<u64>();
// println!("{ans}");
