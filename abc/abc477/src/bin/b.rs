// algo: brute-force
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

// 各人について「自分以外の全員との距離が D 以上か」を判定する問題。N <= 100 なので、
// 全ペアを調べても 100 × 100 = 10^4 回で済む。工夫はいらず、定義をそのまま書く。
//
// 型: X_i は 10^9 以下で usize に収まるが、x[i] - x[j] を usize のまま計算すると
// x[i] < x[j] のときにアンダーフローする (debug では panic、release では巨大な値)。
// そこで i64 に直してから abs を取っている。差の絶対値は 10^9 未満なので i64 で
// 十分に収まる。
//
// 自分自身 (i == j) を除くのを忘れると、距離 0 < D なので全員が不合格になり、
// 答えは常に 0 人になる。サンプル 2 (全員同じ座標) は答えが 0 なので、この誤りでも
// 偶然通ってしまう。
//
// 出力: K = 0 のとき 2 行目は空行が必要。空の Vec を join すると "" になり、
// println! が改行だけを出すので、場合分けしなくても空行になる。
// i を昇順に回して push しているので、ans はもともと昇順で、sort_unstable は
// 実質何もしていない。

#[fastout]
fn main() {
    input! {
        n: usize,
        d: usize,
        x: [usize; n],
    };

    let mut ans: Vec<usize> = vec![];

    for i in 0..n {
        let mut ok = true;
        for j in 0..n {
            if i != j {
                let sub = (x[i] as i64 - x[j] as i64).abs();
                if (sub as usize) < d {
                    ok = false;
                    break;
                }
            }
        }
        if ok {
            ans.push(i + 1);
        }
    }

    println!("{}", ans.len());
    ans.sort_unstable();
    let s = ans.iter().join(" ");
    println!("{s}");
}

// alt: usize::abs_diff で差の絶対値を直接取り、判定を all で書く版。i64 へのキャストと
//      フラグ変数・break が消え、「自分以外の全員が D 以上」という定義がそのまま式になる
// let ans: Vec<usize> = (0..n)
//     .filter(|&i| (0..n).all(|j| i == j || x[i].abs_diff(x[j]) >= d))
//     .map(|i| i + 1)
//     .collect();

// alt: 座標順に並べると、一番近い人は必ず左右どちらかの隣にいる。そのため左右の隣との
//      距離だけ見ればよく、O(N log N) になる。N が 10^5 規模に増えても通る形
//      (本問では不要)。元の番号で出力するので最後にソートし直す
// let mut idx: Vec<usize> = (0..n).collect();
// idx.sort_unstable_by_key(|&i| x[i]);
// let mut ans: Vec<usize> = (0..n)
//     .filter(|&k| {
//         let left = k == 0 || x[idx[k]] - x[idx[k - 1]] >= d;
//         let right = k == n - 1 || x[idx[k + 1]] - x[idx[k]] >= d;
//         left && right
//     })
//     .map(|k| idx[k] + 1)
//     .collect();
// ans.sort_unstable();
