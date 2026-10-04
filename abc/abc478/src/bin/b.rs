// algo: brute-force
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

// N 種類から相異なる 3 種類を選び、価格 (= 番号) の和が V 以下という条件のもとで
// 嬉しさの和を最大化する問題。N <= 100 なので 3 つ組を全部試せる。
//
// permutations(3) は順序を区別するので 100 * 99 * 98 ≈ 9.7 * 10^5 通りを見る。
// 同じ組を 3! = 6 回ずつ数えているが、最大値を取るだけなので重複しても答えは
// 変わらず、10^6 程度なので間に合う。
//
// 価格は 0-indexed の添字に +1 したもの。enumerate の添字をそのまま足すと 1 種類に
// つき 1 ずつ安く見積もってしまい、本来は予算オーバーの組が通ってしまう。
//
// 和は最大でも 3 * 10^6 なので usize で十分。V >= 6 = 1 + 2 + 3 なので条件を満たす
// 組が必ずあり、max の初期値 0 が答えとして出ることはない。

#[fastout]
fn main() {
    input! {
        n: usize,
        v: usize,
        w: [usize; n],
    };

    let perms = w.iter().enumerate().permutations(3);

    let mut max = 0usize;
    for p in perms {
        let mut total_p = 0usize;
        let mut total_v = 0usize;
        for (price, &value) in p {
            total_p += price + 1;
            total_v += value;
        }

        if total_p <= v {
            max = max.max(total_v);
        }
    }

    println!("{max}");
}

// alt: tuple_combinations で i < j < k の組だけを作れば重複が消え、見る数は 1/6 の
//      C(100, 3) ≈ 1.6 * 10^5 になる。番号を 1..=n で回すので価格の +1 も要らず、
//      Vec の確保も無い。filter → map → max で宣言的に書ける
// let ans = (1..=n)
//     .tuple_combinations()
//     .filter(|&(i, j, k)| i + j + k <= v)
//     .map(|(i, j, k)| w[i - 1] + w[j - 1] + w[k - 1])
//     .max()
//     .unwrap();
// println!("{ans}");
