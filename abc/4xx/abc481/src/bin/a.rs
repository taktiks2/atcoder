// algo: math
// solve: self 2026-10-24
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

// A+B の桁数を答える問題。制約 1 <= A, B <= 9 から A+B は 2〜18 に収まるので、
// 桁数は 1 か 2 のどちらかしかない。境界は 10 (= 最小の 2 桁) で、a + b > 9 と
// a + b >= 10 は同じ。サンプル 3 の「7 3 → 10」がちょうどこの境界を突いている
// (> 10 と書くと 10 を 1 桁扱いして落ちる)。
//
// 一般の桁数なら to_string().len() や「10 で割れなくなるまで割る」が要るが、
// 本問は値域が狭いので分岐一本で足りる。

#[fastout]
fn main() {
    input! {
        a: usize,
        b: usize,
    };

    let mut ans = if a + b > 9 { 2 } else { 1 };

    println!("{ans}");
}

// alt: 構造はそのまま。
//      - 再代入しない ans から mut を外した (unused_mut の警告が消える)
// let ans = if a + b > 9 { 2 } else { 1 };
// println!("{ans}");

// alt: 値域に依存しない書き方。文字列にして長さを取るだけなので、A, B の制約が
//      広がっても (例えば 10^18 まで) そのまま通る。境界 10 を自分で書かずに済むので
//      > と >= の取り違えも起きない
// println!("{}", (a + b).to_string().len());
