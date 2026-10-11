// algo: counting, brute-force
// solve: copy 2026-10-11
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

// 全ペア (i < j) の (A_i - A_j)^2 の総和。N <= 3*10^5 なのでペアを直接回すと
// N^2/2 ≈ 4.5*10^10 で TLE。
//
// 鍵は |A_i| <= 200 で、値の種類が 401 通りしかないこと。式は値だけで決まるので、
// 「値 x が c_x 個、値 y が c_y 個」なら、その組み合わせの寄与は (x - y)^2 * c_x * c_y
// にまとめられる。添字のペアの代わりに値のペアを回せばよく、401^2/2 ≈ 8*10^4 回で済む。
// count は A_i + 200 を添字にして、負の値を 0..=400 に寄せる。
//
// 同じ値どうしのペアは差が 0 なので数えなくてよく、j は i + 1 から始める。
// (i, j) と (j, i) を両方数えると答えがちょうど 2 倍になる。
//
// 型: 最大は半分が -200、半分が 200 のとき (1.5*10^5)^2 * 400^2 = 3.6*10^15。
// i32 (2.1*10^9) では溢れるので i64 にする。count も i64 にしておけば掛け算の途中で溢れない。

#[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [i64; n],
    };

    let mut count = vec![0i64; 401];

    for ai in a {
        let index = (ai + 200) as usize;
        count[index] += 1;
    }

    let mut ans = 0i64;
    for i in 0..401 {
        for j in (i + 1)..401 {
            let diff = (j - i) as i64;
            ans += diff * diff * count[i] * count[j];
        }
    }

    println!("{ans}");
}

// alt: 構造はそのまま (値ごとに数える + 値のペアを回す)
// - `mut a` → `a` (書き換えていない)
// - index 変数を書かず、添字に直接入れる
// - 二重ループ → tuple_combinations (i < j のペアを列挙) + sum
// input! { n: usize, a: [i64; n] }
// let mut count = vec![0i64; 401];
// for ai in a {
//     count[(ai + 200) as usize] += 1;
// }
// let ans: i64 = (0..401)
//     .tuple_combinations()
//     .map(|(i, j)| (j - i) as i64 * (j - i) as i64 * count[i] * count[j])
//     .sum();
// println!("{ans}");

// alt: 式変形で O(N)。値域の制約 (|A_i| <= 200) を使わないので、値が大きくても通る
// 順序付きペア全体の和 Σ_i Σ_j (a_i - a_j)^2 を展開すると 2N Σa^2 - 2(Σa)^2。
// i = j の項は 0、(i, j) と (j, i) は同じ値なので、半分にすると i < j の和は N Σa^2 - (Σa)^2
// input! { n: usize, a: [i64; n] }
// let s: i64 = a.iter().sum();
// let s2: i64 = a.iter().map(|x| x * x).sum();
// println!("{}", n as i64 * s2 - s * s);
