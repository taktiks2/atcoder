// algo: grid, sort
// solve: hint 2026-11-10
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

// H 行 W 列の図形 S の列を並べ替えて T と一致させられるかを判定する問題。H×W ≤ 4×10^5。
// 列の並べ替えを全部試すと W! 通りで論外。
//
// 列の並べ替えは「列を 1 本ずつ丸ごと動かす」だけで、列の中身は変わらない。よって
// 一致させられる ⇔ S の列の集まりと T の列の集まりが重複込みで等しい。両方の列を同じ
// 規則 (辞書順) でソートすれば、等しい集まりは必ず同じ配列になり、違えばどこかで必ずずれる。
// 「どの列をどこへ対応させるか」を探す必要はなく、ソート後に同じ位置にあるもの同士が対応する。
//
// 行ごとの # の個数を比べるのは誤り。S = "#." / ".#"、T = "#." / "#." は各行の # が 1 個ずつで
// 揃うが、列の集まりは {"#.", ".#"} と {"##", ".."} で違うので答えは No。
//
// 計算量: 列の抽出 O(HW)、ソートは比較 1 回が O(H) で O(HW log W) ≈ 4×10^5 × 19、
// 最後の比較 O(HW)。H と W はどちらも単独で最大 4×10^5 になりうるので、固定長の 2 次元配列は使えない。

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
        t: [Chars; h],
    };

    let mut s_columns: Vec<Vec<char>> = vec![];
    let mut t_columns: Vec<Vec<char>> = vec![];

    for i in 0..w {
        s_columns.push(s.iter().map(|x| x[i]).collect());
        t_columns.push(t.iter().map(|x| x[i]).collect());
    }

    s_columns.sort_unstable();
    t_columns.sort_unstable();

    let ans = s_columns
        .iter()
        .zip(&t_columns)
        .all(|(s_c, t_c)| s_c == t_c);

    println!("{}", if ans { "Yes" } else { "No" });
}

// alt: リファクタリング版。構造はそのまま
//      - vec![vec![]; w] + push → Vec::with_capacity(w)。空の列 w 本が混ざらなくなる
//      - zip + all → == 1 回。長さの違いも含めて比べられ、短い
// let mut s_columns: Vec<Vec<char>> = Vec::with_capacity(w);
// let mut t_columns: Vec<Vec<char>> = Vec::with_capacity(w);
//
// for i in 0..w {
//     s_columns.push(s.iter().map(|x| x[i]).collect());
//     t_columns.push(t.iter().map(|x| x[i]).collect());
// }
//
// s_columns.sort_unstable();
// t_columns.sort_unstable();
//
// println!("{}", if s_columns == t_columns { "Yes" } else { "No" });

// alt: HashMap で列の出現回数を数える。ソートの log が消えて O(HW)。「重複込みで等しい」を
//      S で +1、T で -1 して全部 0 に戻るかで直接表せる
// let mut cnt: HashMap<Vec<char>, i64> = HashMap::new();
// for i in 0..w {
//     *cnt.entry(s.iter().map(|x| x[i]).collect()).or_insert(0) += 1;
//     *cnt.entry(t.iter().map(|x| x[i]).collect()).or_insert(0) -= 1;
// }
// let ans = cnt.values().all(|&c| c == 0);
// println!("{}", if ans { "Yes" } else { "No" });
