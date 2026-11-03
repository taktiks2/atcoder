// algo: binary-search
// solve: hint 2026-11-03
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

// 区間 [L, R] に値 X が何回現れるかを Q 回答える問題。N, Q <= 2*10^5。
// 毎回区間を舐めると最悪 N*Q = 4*10^10 で間に合わない。値ごとの累積和
// cnt[x][i] を持てば O(1) で答えられるが、表が N*N = 4*10^10 要素でメモリに載らない。
//
// そこで「値 → その値が現れる位置の一覧」を持つ。一覧の長さの総和はちょうど N なので
// メモリは O(N)。A を前から走査して push するだけで各一覧は自動的に昇順になり、
// ソートは要らない。
//
// 昇順の一覧では [L, R] に入る要素は連続した 1 区間になる。だから個数は
// 「R 以下の個数」-「L 未満の個数」で、どちらも partition_point の返り値そのもの
// (述語 v <= r / v < l は前半 true・後半 false に分かれる)。全体で O(N + Q log N)。
//
// 落とし穴は添字の基準。A_i を Usize1 で 0-indexed にしたので list の長さは n で足りる
// (1-indexed のまま X を添字にするなら n+1 要る)。位置 i も L, R も 0-indexed に
// 揃えているので比較がずれない。X が一度も現れない場合は空の一覧で 0 - 0 = 0 になり、
// 特別扱いは不要。答えは高々 N で usize で十分。

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [Usize1; n],
        q: usize,
    };

    let mut list: Vec<Vec<usize>> = vec![vec![]; n];

    for (i, &ai) in a.iter().enumerate() {
        list[ai].push(i);
    }

    for _ in 0..q {
        input! {
            l: Usize1,
            r: Usize1,
            x: Usize1,
        }

        let start = list[x].partition_point(|&v| v < l);
        let last = list[x].partition_point(|&v| v <= r);

        println!("{}", last - start);
    }
}

// alt: 構造はそのまま。冗長な部分だけ削った版
//      - list[x] を 2 回引いていたのを let pos = &list[x] の 1 回に
//      - Vec<Vec<usize>> の型注釈は push する値から推論されるので省略
//      - start / last の一時変数をやめ、差を println! に直接書く
// let mut list = vec![vec![]; n];
// for (i, &ai) in a.iter().enumerate() {
//     list[ai].push(i);
// }
// for _ in 0..q {
//     input! {
//         l: Usize1,
//         r: Usize1,
//         x: Usize1,
//     }
//     let pos = &list[x];
//     println!("{}", pos.partition_point(|&v| v <= r) - pos.partition_point(|&v| v < l));
// }

// alt: オフライン + 累積カウントで log を消す版。O(N + Q)。答え = (先頭 R 個中の x の数)
//      - (先頭 L-1 個中の x の数) と分解し、クエリを「先頭 p 個の時点で cnt[x] を見る」
//      イベント 2 つに割って p の小さい順に掃く。cnt は 1 本の配列を使い回すので、
//      値ごとの累積和表 (N*N) を持たずに済む。クエリを先読みできることが前提
// input! {
//     n: usize,
//     a: [Usize1; n],
//     q: usize,
//     lrx: [(usize, usize, Usize1); q],
// };
// let mut ev = vec![vec![]; n + 1]; // ev[p] = 先頭 p 個の時点で見るクエリ (番号, x, 符号)
// for (qi, &(l, r, x)) in lrx.iter().enumerate() {
//     ev[l - 1].push((qi, x, -1i64));
//     ev[r].push((qi, x, 1));
// }
// let mut cnt = vec![0i64; n];
// let mut ans = vec![0i64; q];
// for p in 0..=n {
//     if p > 0 {
//         cnt[a[p - 1]] += 1;
//     }
//     for &(qi, x, sign) in &ev[p] {
//         ans[qi] += sign * cnt[x];
//     }
// }
// for v in ans {
//     println!("{v}");
// }
