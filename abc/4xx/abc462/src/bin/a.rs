// algo: string
// solve: self 2026-11-22
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

// 英小文字と数字からなる S から、数字だけを元の順序のまま抜き出して繋げる問題。
// filter は要素の順序を保つので、先頭から「数字か」で絞って連結すればそれが答え。
// |S| <= 50 なので計算量は考えなくてよい。
//
// 落とし穴は数値として扱おうとすること。抜き出した結果を parse すると
//   - 数字が 1 つも無いとき (サンプル 2: codequeen) は空文字列で、parse がエラーになる
//   - "007" のような先頭の 0 が消える
//   - 最大 50 桁になり得るので u64 (約 1.8 * 10^19 = 20 桁) にも u128 にも収まらない
// の 3 通りで壊れる。最後まで文字列のまま持つのが正しい。
//
// 数字が無いときは空文字列を println! するので空行が 1 行出る。これが期待出力。

#[fastout]
fn main() {
    input! {
        s: Chars,
    };

    let ans = s.iter().filter(|&&x| x.is_numeric()).join("");

    println!("{ans}");
}

// alt: 構造はそのまま (Chars → filter → 連結)。Rust らしさの整理だけ
//      - is_numeric → is_ascii_digit。is_numeric は Unicode の数字全般 ('٣' や '½' など)
//        も true にするので、「'0'..='9' か」という意図どおりの is_ascii_digit にする
//        (本問の入力は英小文字と数字だけなので結果は同じ)
//      - itertools の join("") → collect::<String>()。区切り文字が空なら標準ライブラリ
//        だけで char の列を String に集められる
//      - |&&x| の二重参照外しも不要 (メソッド呼び出しは自動で参照を外す)
// let ans: String = s.iter().filter(|c| c.is_ascii_digit()).collect();
// println!("{ans}");

// alt: String で受けて retain でその場で絞る版。新しい文字列を作らず、「条件を満たす
//      文字だけ残す」という問題文の操作がそのまま 1 行になる。Vec<char> を経由しない
//      ぶん短い
// input! { mut s: String }
// s.retain(|c| c.is_ascii_digit());
// println!("{s}");
