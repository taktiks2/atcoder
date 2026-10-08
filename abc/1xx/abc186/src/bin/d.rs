// algo: sort, math
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

// 全ペア (i, j) の |A_i - A_j| の総和を求める問題。ペアを全部見ると N(N-1)/2 ≈ 2×10^10
// 回で、N = 2×10^5 では TLE。ペアごとではなく「各値が総和に何回 + / - で効くか」で数える。
//
// |A_i - A_j| はペアについて対称なので、並べ替えても答えは変わらない (i < j の順序は
// ペアの集合を決めるためだけのもの)。昇順にソートすると絶対値が外れて、どのペアも
// 「右の値 - 左の値」になる。ソート後 0-indexed で i 番目の値は、左にいる i 個と組むと
// 大きい側 (+)、右にいる N-1-i 個と組むと小さい側 (-) になるので、総和への寄与は
// a[i] * (i - (N-1-i))。これを全 i で足せば O(N log N) (ソート) + O(N)。
//
// 型は i64。値に負があるので符号付きが必須で、答えの最大は「-10^8 と 10^8 が 10^5 個ずつ」
// のとき 10^10 ペア × 2×10^8 = 2×10^18。i32 (2.1×10^9) は論外だが i64 (9.2×10^18) には
// 収まる。途中の a[i] * 係数 も高々 10^8 × 2×10^5 = 2×10^13。
//
// 落とし穴は係数を usize のまま計算すること。i - (n - 1 - i) は左半分で負になり、usize だと
// アンダーフローで panic (debug) / 巨大な値 (release) になる。left / right を i64 に
// キャストしてから引いているのはそのため。
// 出力は 1 行なので #[fastout] は本問では効いていない。

#[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [i64; n],
    };

    a.sort_unstable();

    let mut total = 0i64;

    for (i, &ai) in a.iter().enumerate() {
        let left = i as i64;
        let right = n as i64 - 1 - left;

        total += ai * (left - right);
    }

    println!("{total}");
}

// alt: 累積和で「自分より左の値の合計」を持ちながら回す版。a[j] を右端とするペアの差の和は
//      a[j] * j - (a[0] + ... + a[j-1]) なので、係数を考えずに済み「左の全員との差を一気に足す」
//      という意味がそのまま式に出る。計算量は同じ O(N log N)
// a.sort_unstable();
// let mut ans = 0i64;
// let mut sum = 0i64; // a[0..j] の合計
// for (j, &aj) in a.iter().enumerate() {
//     ans += aj * j as i64 - sum;
//     sum += aj;
// }
// println!("{ans}");

// alt: 寄与の式 a[i] * (2i - (N-1)) を map + sum で畳んだ版。可変の total が消えて、
//      「各値の寄与の総和」という考え方が一行の式になる。最短
// a.sort_unstable();
// let ans: i64 = a
//     .iter()
//     .enumerate()
//     .map(|(i, &x)| x * (2 * i as i64 - (n as i64 - 1)))
//     .sum();
// println!("{ans}");
