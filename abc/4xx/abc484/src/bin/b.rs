// algo: sort
// solve: self 2026-11-14
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

// 卓上に残った K 個のボールのうち最小の番号を答える問題。ただし 9 番が無ければ -1。
// 必要なのは「9 番があるか」と「最小値」の 2 つだけ。
//
// 昇順にソートすると、最小値は先頭 a[0] に、最大値は末尾 a[k - 1] に来る。番号は 1 以上
// 9 以下なので、9 番があるなら必ず最大値 = 末尾になる。よって「9 番があるか」は末尾が
// 9 かどうかだけで判定でき、全体を探す必要がない。
//
// 9 番しか残っていない (K = 1, A = [9]) ときは a[0] と a[k - 1] が同じ要素で、答えは 9。
// 「9 番を除いた中の最小」と読み違えると、ここで答えが無くなる。サンプルには無い形。
//
// 制約 1 <= K から a は空にならないので a[0] も a[k - 1] も panic しない (k - 1 の
// アンダーフローも起きない)。K <= 9 なのでソートの計算量は問題にならない。型は -1 を
// 出すために符号付きの i32 にしている。

#[fastout]
fn main() {
    input! {
        k: usize,
        mut a: [i32; k],
    };

    a.sort_unstable();

    let ans = if a[k - 1] == 9 { a[0] } else { -1 };

    println!("{ans}");
}

// alt: リファクタリング版なし — ソート 1 行と if 式 1 行だけで、削れる冗長な部分が無い

// alt: ソートせずに「9 があるか」と「最小値」をそのまま聞く版。問題文の 2 つの条件が
//      contains と min にそのまま対応するので読みやすい。並べ替えないので入力の mut も
//      要らず、計算量も O(K log K) → O(K) になる (K <= 9 なので速度差は出ない)
// input! {
//     k: usize,
//     a: [i32; k],
// };
// let ans = if a.contains(&9) { *a.iter().min().unwrap() } else { -1 };
// println!("{ans}");
