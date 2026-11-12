// algo: math
// solve: self 2026-11-13
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

// N 両編成で前から K 両目が、後ろから何両目かを答える問題。答えは X = N - K + 1。
// 前から K 両目の後ろには N - K 両あるので、自分を含めて後ろから数えると N - K + 1 両目になる。
// 端の値で確かめると、K = 1 (先頭) なら N、K = N (最後尾) なら 1 で合っている。
//
// +1 を忘れて N - K を出すとオフバイワンになる。sample1 の 5 2 で 3 が出るのでサンプルで気付ける。
//
// 型は usize で、引き算のアンダーフローを気にする必要がある。制約 K <= N から N - K >= 0 なので
// n - k + 1 の順でも panic しない。ただ K > N があり得る問題で同じ式を書くと、debug ビルドでは
// panic、release ビルドでは wrap して巨大な値になる。n + 1 - k の順 (先に足してから引く) に
// しておくと、K = N + 1 まで安全になる。

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
    };

    let mut ans = n - k + 1;

    println!("{ans}");
}

// alt: 構造はそのまま (ans を作って出す)。冗長な部分だけ削った
//      - 再代入しないので let mut → let (unused_mut の警告が消える)
//      - 足してから引く順にして、usize のアンダーフローに一段強くした
// let ans = n + 1 - k;
// println!("{ans}");
