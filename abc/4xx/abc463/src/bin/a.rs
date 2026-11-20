// algo: math
// solve: self 2026-11-20
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

// 横 X・縦 Y の画像が X : Y = 16 : 9 かを判定する問題。比 X/Y = 16/9 の分母を
// 払って X * 9 = Y * 16 と整数だけの比較に直す。X, Y >= 1 で 0 割りも無く、
// 元の条件と完全に同値。
//
// 比のまま割り算で書くと壊れる。整数除算で x / y == 16 / 9 とすると右辺は 1 に
// 切り捨てられ、(X, Y) = (10, 9) でも 10 / 9 = 1 で Yes になってしまう。f64 に
// 直して比べるのも、割り切れない値どうしの == になるので避ける。
//
// 向きにも注意。サンプル 3 (108 192) は 9 : 16 の縦長で No。掛ける相手を取り違えて
// x * 16 == y * 9 と書くとここで Yes を返す。
//
// X, Y <= 1000 なので X * 9 <= 9000, Y * 16 <= 16000。どの整数型でも溢れない。

#[fastout]
fn main() {
    input! {
        x: usize,
        y: usize,
    };

    let ans = if x * 9 == y * 16 { "Yes" } else { "No" };

    println!("{ans}");
}

// alt: 構造はそのまま (分母を払って整数比較 → Yes/No)。短さの整理だけ
//      - ans に束縛せず、if 式をそのまま println! に渡す
// println!("{}", if x * 9 == y * 16 { "Yes" } else { "No" });

// alt: 掛け算ではなく「同じ整数 k で X = 16k, Y = 9k と書けるか」で判定する版。
//      16 と 9 は互いに素なので X * 9 = Y * 16 なら X は 16 の倍数、Y は 9 の倍数で、
//      商が一致することと同値。積を作らないので X, Y が巨大でも溢れないのが利点
//      (本問の制約では不要で、式が長くなるぶん実戦では掛け算の方が素直)
// let ok = x % 16 == 0 && y % 9 == 0 && x / 16 == y / 9;
// println!("{}", if ok { "Yes" } else { "No" });
