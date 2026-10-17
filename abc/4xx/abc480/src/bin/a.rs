// algo: grid
// solve: self 2026-10-17
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

// 2x2 グリッドで A のマスと D のマスが辺で隣接しているかを判定する問題。
// 2x2 では 2 マスの関係は「辺で隣接（横 2 組 + 縦 2 組）」か「対角（2 組）」の 2 通りしか
// ない。隣接する 4 組それぞれについて「両方が A か D か」を見れば答えになる。
//
// 「両方が A または D」で {A, D} の組だと言い切れるのは、4 マスの文字が互いに異なる
// （A も D も 1 個ずつしかない）という制約のおかげ。同じ文字が複数あり得る問題なら
// A, A の組も Yes にしてしまう。
//
// 4 本の if は 上の行 / 下の行 / 右の列 / 左の列（s2[0] と s1[0] は順序が逆なだけ）で、
// 横 2 + 縦 2 をちょうど覆っている。対角 (s1[0], s2[1]) と (s1[1], s2[0]) は含めない。
// 落とし穴: サンプル 1 は左の列、サンプル 3 は右の列で Yes になるケースで、横に並ぶ
// ケースはサンプルに無い。行の if 2 本を書き漏らしてもサンプルは全部通るのに WA になる。

#[fastout]
fn main() {
    input! {
        s1: Chars,
        s2: Chars,
    };

    let mut ans = "No";
    if (s1[0] == 'A' || s1[0] == 'D') && (s1[1] == 'A' || s1[1] == 'D') {
        ans = "Yes";
    }

    if (s2[0] == 'A' || s2[0] == 'D') && (s2[1] == 'A' || s2[1] == 'D') {
        ans = "Yes";
    }

    if (s1[1] == 'A' || s1[1] == 'D') && (s2[1] == 'A' || s2[1] == 'D') {
        ans = "Yes";
    }

    if (s2[0] == 'A' || s2[0] == 'D') && (s1[0] == 'A' || s1[0] == 'D') {
        ans = "Yes";
    }

    println!("{ans}");
}

// alt: 構造はそのまま（隣接する 4 組を個別に調べる）。冗長な部分だけ削った版
//      - `c == 'A' || c == 'D'` の 8 回の繰り返し → クロージャ ad 1 つ
//      - mut ans + if 4 本 → 4 組を || でつないだ bool 式 1 本
// let ad = |c: char| c == 'A' || c == 'D';
// let ans = (ad(s1[0]) && ad(s1[1]))
//     || (ad(s2[0]) && ad(s2[1]))
//     || (ad(s1[0]) && ad(s2[0]))
//     || (ad(s1[1]) && ad(s2[1]));
// println!("{}", if ans { "Yes" } else { "No" });

// alt: 補集合で考える版。2x2 で隣接しないのは対角の 2 組だけなので、そちらが {A, D} かを
//      調べて反転する。調べる組が 4 → 2 に減り、組の書き漏らしが起きにくい（短さ）
// let ad = |c: char| c == 'A' || c == 'D';
// let diag = (ad(s1[0]) && ad(s2[1])) || (ad(s1[1]) && ad(s2[0]));
// println!("{}", if diag { "No" } else { "Yes" });

// alt: A と D の座標を探し、マンハッタン距離が 1 かで判定する版。2x2 専用の場合分けに頼らず、
//      H x W のグリッドにそのまま一般化できる。入力を [Chars; 2] で受ける前提
// input! { s: [Chars; 2] };
// let pos = |c: char| {
//     (0..2)
//         .flat_map(|i| (0..2).map(move |j| (i, j)))
//         .find(|&(i, j)| s[i][j] == c)
//         .unwrap()
// };
// let ((ai, aj), (di, dj)) = (pos('A'), pos('D'));
// let dist = ai.abs_diff(di) + aj.abs_diff(dj);
// println!("{}", if dist == 1 { "Yes" } else { "No" });
