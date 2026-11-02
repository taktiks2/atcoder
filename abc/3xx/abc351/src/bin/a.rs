// algo: math
// solve: self 2026-11-02
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

// 9 回表終了時点の高橋の合計 S_A と、8 回裏までの青木の合計 S_B が分かっている。
// 9 回裏に青木が x 点取ると最終スコアは S_A 対 S_B + x。同点は引き分けなので、
// 勝つ条件は S_B + x > S_A、つまり x >= S_A - S_B + 1 で、その最小値がそのまま答え。
// 「9 回裏」という野球の設定は飾りで、合計の差を出すだけの問題になる。
//
// 落とし穴は +1 の付け忘れ。「追いつく」点数 S_A - S_B を出すと同点 = 引き分けで、
// サンプル 2 (全部 0) で 0 を出して落ちる。「勝つ」を先に不等号 (>) に直してから式にする。
//
// usize の引き算 a_total - b_total は、制約 S_A >= S_B があるので負にならず安全。
// この保証が無い問題で usize のまま引くと、debug ではパニック、release ではラップして
// 巨大な値になる。合計は高々 99 * 9 = 891 なので、どの整数型でも溢れない。

#[fastout]
fn main() {
    input! {
        a: [usize; 9],
        b: [usize; 8],
    };

    let a_total: usize = a.iter().sum();
    let b_total: usize = b.iter().sum();

    println!("{}", a_total - b_total + 1);
}

// alt: 構造はそのまま (合計 2 本 → 差 + 1)。一時変数を置かず 1 行にした短い版
//      - let a_total / b_total の 2 行 → turbofish で式に直書き
//      - sum は戻り値型を推論できないので、変数の型注釈が無いなら ::<usize> が要る
// println!("{}", a.iter().sum::<usize>() - b.iter().sum::<usize>() + 1);
