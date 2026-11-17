// algo: brute-force
// solve: self 2026-11-17
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

// N 個の選択肢のどれを選んでも嬉しさ X_i が負になるかを判定する問題。
//
// 「どれを選んでも負」は「全ての i について X_i < 0」と同じなので、全要素を見て
// all で判定すればよい。N <= 10 なので計算量は気にしなくてよい。
//
// 落とし穴は 0 の扱い。0 は負ではないので、0 が 1 つでもあれば No になる
// (サンプル 3 の 0 -2 0 -1)。条件を x <= 0 と書くとここで WA になる。
//
// all は false になった時点で打ち切る (短絡評価)。空の列に対しては true を返すが、
// N >= 2 なので本問では関係しない。|X_i| <= 100 なので型は i32 でも足りる。

#[fastout]
fn main() {
    input! {
        n: usize,
        x: [i64; n],
    };

    let ans = if x.iter().all(|&x| x < 0) {
        "Yes"
    } else {
        "No"
    };

    println!("{ans}");
}

// alt: 構造はそのまま、クロージャの引数名だけ変えたリファクタリング版。
//      - `|&x|` は外側の配列 `x` を隠す (シャドーイング) ので `|&v|` にする
//      - 動作は同じだが、クロージャ内の x が配列なのか要素なのか迷わなくなる
// let ans = if x.iter().all(|&v| v < 0) { "Yes" } else { "No" };

// alt: 最大値だけ見る版。「全部が負」は「最大値が負」と同値なので、判定を 1 つの値に
//      集約できる。条件が「全部が K 未満」「全部が K 以上 (→ 最小値)」に変わっても同じ形で
//      書けるのが利点。max は Option を返すので unwrap が要る (N >= 2 なので空にならない)。
//      all と違って途中で打ち切らず必ず N 個見るが、N <= 10 では差は出ない
// let ans = if *x.iter().max().unwrap() < 0 { "Yes" } else { "No" };
