// algo: math
// solve: self 2026-11-19
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

// A > B * 2/3 かどうかを判定する問題。2/3 は整数にならないので、両辺を 3 倍して
// A * 3 > B * 2 と整数だけの比較に直す。3 は正なので不等号の向きは変わらず、
// 元の条件と完全に同値。浮動小数の誤差を考える必要が無くなる。
//
// 等号の扱いが唯一の落とし穴。サンプル 3 (666 999) は 666 * 3 = 1998 = 999 * 2 で
// ちょうど 2/3 になり、「より大きい」ではないので No。>= と書くとここで WA になる。
//
// 割る順序にも注意。B / 3 * 2 と先に割ると切り捨てが早すぎて、(A, B) = (1, 2) で
// 1 > 2 / 3 * 2 = 0 となり Yes を返してしまう (正しくは 3 > 4 が偽で No)。
//
// A, B <= 1000 なので A * 3 <= 3000, B * 2 <= 2000。どの整数型でも溢れない。

#[fastout]
fn main() {
    input! {
        a: usize,
        b: usize,
    };

    let ans = if (a * 3) > (b * 2) { "Yes" } else { "No" };

    println!("{ans}");
}

// alt: 構造はそのまま (3 倍して整数比較 → Yes/No)。Rust らしさ・短さの整理だけ
//      - 掛け算は比較より優先度が高いので (a * 3) > (b * 2) の括弧は不要
//      - ans に束縛せず、if 式をそのまま println! に渡す
// println!("{}", if a * 3 > b * 2 { "Yes" } else { "No" });

// alt: 移項せず問題文の式の形のまま、整数除算の切り捨てで書く版。整数 A については
//      A > x と A > floor(x) が同値なので、B * 2 / 3 を切り捨てても判定は変わらない。
//      「掛けてから割る」順序が必須 (先に割る B / 3 * 2 は上の通り壊れる)。式変形が
//      要らない代わりに正しさの根拠が一段深いので、実戦では 3 倍する方が安全
// println!("{}", if a > b * 2 / 3 { "Yes" } else { "No" });
