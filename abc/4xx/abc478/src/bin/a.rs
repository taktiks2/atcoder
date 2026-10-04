// algo: simulation
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

// N 人に M 個を 1 個ずつ順に配る問題。問題文の通りに M 回回しても、M <= 10^4 なので
// 一瞬で終わる。i 個目のブドウを受け取るのは i % N 番の人 (0-indexed) で、一周して
// 戻る処理が剰余で書ける。
//
// 回数が M に比例するのが気にならないのは制約が小さいからで、M が 10^18 なら
// 回せない。そのときは下の alt の閉じた式を使う。

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
    };

    let mut arr: Vec<usize> = vec![0; n];

    for i in 0..m {
        let who = i % n;
        arr[who] += 1;
    }

    for j in arr {
        println!("{j}");
    }
}

// alt: 回さずに式で出す。全員が M / N 個ずつもらい、余りの M % N 個は先頭の
//      M % N 人に 1 個ずつ行く。O(N) で M に依存しないので、M が巨大でも使える
// for i in 0..n {
//     println!("{}", m / n + usize::from(i < m % n));
// }
