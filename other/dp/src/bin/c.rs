// algo: dp
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        abc: [[i64; 3]; n],
    };

    let mut dp = vec![[0i64; 3]; n];

    for j in 0..3 {
        dp[0][j] = abc[0][j];
    }

    for i in 1..n {
        for j in 0..3 {
            dp[i][j] = (0..3)
                .filter(|&x| x != j)
                .map(|x| dp[i - 1][x] + abc[i][j])
                .max()
                .unwrap()
        }
    }

    let ans = dp[n - 1].iter().max().unwrap();

    println!("{ans}");
}

// alt: 参照するのは直前の 1 行だけなので、全体の表を持たず長さ 3 の配列を畳み込む。
//      メモリが O(N) -> O(1)。初期値 [0; 3] から 1 日目も同じ遷移で処理でき、
//      「1 日目には制約が無い」という場合分け (dp[0] の初期化) がそのまま消える
// let dp = abc.iter().fold([0i64; 3], |prev, a| {
//     std::array::from_fn(|j| (0..3).filter(|&k| k != j).map(|k| prev[k]).max().unwrap() + a[j])
// });
// println!("{}", dp.iter().max().unwrap());

// alt: 活動が K 種類に増えた場合の形。「自分以外の最大値」を毎回 K 通り走査すると
//      O(NK^2) になるが、前日の最大値とその活動 (best, arg) さえ持てば
//      「j が最大の担い手でなければ最大値、そうなら 2 番目」で O(NK) になる。
//      K = 3 の本問では差は出ないが、状態数が増える DP で効いてくる定石
// let mut prev = vec![0i64; 3];
// for a in &abc {
//     let (i1, i2) = prev
//         .iter()
//         .enumerate()
//         .sorted_by_key(|&(_, v)| std::cmp::Reverse(v))
//         .map(|(i, _)| i)
//         .next_tuple()
//         .unwrap();
//     prev = (0..3)
//         .map(|j| prev[if j == i1 { i2 } else { i1 }] + a[j])
//         .collect();
// }
// println!("{}", prev.iter().max().unwrap());
