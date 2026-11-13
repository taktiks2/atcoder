// algo: brute-force
// solve: self 2026-11-13
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

// 隣接する 3 つ組 (A_i, A_{i+1}, A_{i+2}) のうち、真ん中が両隣より真に大きい「山」の数を数える問題。
// N <= 100 なので、3 つ組 N - 2 個を全部見て O(N) で済む。
//
// windows(3) は長さ 3 の連続部分スライスを左から順に返す。添字 i, i+1, i+2 を自分で書かないので
// 範囲外アクセスやループ上限 (0..n-2) の書き間違いが起きない。N >= 3 なので windows は 1 個以上の
// 窓を返す (N < 3 でも空のイテレータになるだけで panic はしない)。
//
// 比較は両側とも厳密な不等号 (<, >)。等号を含めると sample2 の 1 1 1 2 1 で
// (1,1,1) なども数えてしまい、1 ではなく 3 になる。「真に大きい」を読み落とさないこと。

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };

    let mut ans = 0usize;
    for window in a.windows(3) {
        if window[0] < window[1] && window[1] > window[2] {
            ans += 1;
        }
    }

    println!("{ans}");
}

// alt: 構造はそのまま (windows(3) を for で回して ans に足す)。if の分岐だけ削った
//      - if cond { ans += 1; } → ans += cond as usize (bool は 0/1 にキャストできる)
// let mut ans = 0usize;
// for w in a.windows(3) {
//     ans += (w[0] < w[1] && w[1] > w[2]) as usize;
// }
// println!("{ans}");

// alt: 「条件を満たす窓の個数」を filter + count でそのまま書いた Rust らしい形。
//      可変の ans が消えて 1 式になる
// let ans = a.windows(3).filter(|w| w[0] < w[1] && w[1] > w[2]).count();
// println!("{ans}");

// alt: itertools の tuple_windows を使うと、窓を (l, m, r) のタプルで受け取れる。
//      w[0] / w[1] / w[2] の添字が名前に変わり、「真ん中が両隣より大きい」がそのまま読める
// let ans = a.iter().tuple_windows().filter(|(l, m, r)| l < m && m > r).count();
// println!("{ans}");
