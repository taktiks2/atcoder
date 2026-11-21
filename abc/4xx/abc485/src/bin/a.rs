// algo: math
// solve: self 2026-11-21
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

// 家を 0 として、スーパー A と駅 B が家から見て同じ方角にあるかを判定する問題。
// 東が正なので「同じ方角」=「A と B の符号が同じ」。両方正か両方負かを調べればよい。
//
// A != 0, B != 0 が制約で保証されているので、符号は正か負の 2 通りしか無く、
// 「どちらかが 0 (家と同じ地点)」の扱いを考える必要が無い。
//
// 型は符号付きが必須。入力は -100 まであるので usize で受けると proconio の parse が
// 失敗して RE になる。サンプル 1, 2 に負の値があるので手元で気付ける。
// |A|, |B| <= 100 なので大きさは i32 でも足りる。

#[fastout]
fn main() {
    input! {
        a: i64,
        b: i64,
    };

    let ans = if (a > 0 && b > 0) || (a < 0 && b < 0) {
        "Yes"
    } else {
        "No"
    };
    println!("{ans}");
}

// alt: 構造はそのまま (符号が一致するかを調べる → Yes/No)。短さの整理だけ
//      - A, B != 0 なので「負」は「正でない」と同じ。両方正 or 両方負の 2 節は
//        「正かどうかが一致する」の 1 つの == に畳める
//      - ans に束縛せず、if 式をそのまま println! に渡す
// println!("{}", if (a > 0) == (b > 0) { "Yes" } else { "No" });

// alt: 積の符号で判定する版。同符号どうしの積は正、異符号なら負なので a * b > 0 だけで
//      済み、最短。|a * b| <= 100 * 100 = 10^4 で溢れない。制約が 10^18 級だと i64 でも
//      積が溢れるので、そのときは使えない書き方
// println!("{}", if a * b > 0 { "Yes" } else { "No" });

// alt: signum で符号そのものを取り出して比べる版。signum は正なら 1、負なら -1、
//      0 なら 0 を返すので、「符号が同じか」という問題文の言い方がそのままコードに
//      なる。掛け算をしないので制約が大きくても溢れない
// println!("{}", if a.signum() == b.signum() { "Yes" } else { "No" });
