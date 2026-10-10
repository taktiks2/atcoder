// algo: prefix-sum, brute-force
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

// 最終形は必ず R…RW…W (W の右に R が来ない ⇔ R が全部左に寄っている)。左から k 個を R に
// する境界 k (0..=n) を決めると、ゴールの候補は n+1 通りに尽きる。どれを目指しても条件は
// 満たせるので、全候補のコストの最小が答え。
//
// 境界 k のコストは a = 左 k 個の W の数, b = 右 n-k 個の R の数として max(a, b)。
// - できる: 左の W と右の R を入れ替えて min(a, b) 組を同時に直し、残り |a-b| 個を色変え
//   → min + |差| = max(a, b) 回。入れ替え・色変えはどちらも 1 回なので内訳は答えに影響しない
// - これ未満は無理: どちらの操作も 1 回で a も b も高々 1 しか減らせない
//
// k ごとに a, b を数え直すと O(N^2) = 4*10^10 で TLE。W と R の累積和を持てば
// a = w[k], b = r[n] - r[k] がそれぞれ O(1) で出て、全体 O(N)。値は N <= 2*10^5 以下なので usize で十分。
//
// 落とし穴: k のループを 0..n にすると「全部 R」(k = n) が漏れる。0..=n にする。
// 色変えだけで直す a + b はサンプル 1 (WWRR → 4、正解 2) で落ちる。

#[fastout]
fn main() {
    input! {
        n: usize,
        c: Chars,
    };

    let mut w = vec![0usize; n + 1];
    let mut r = vec![0usize; n + 1];

    for i in 0..n {
        w[i + 1] = w[i] + (c[i] == 'W') as usize;
        r[i + 1] = r[i] + (c[i] == 'R') as usize;
    }

    let mut ans = usize::MAX;

    for k in 0..=n {
        let a = w[k];
        let b = r[n] - r[k];
        let cost = a.max(b);
        ans = ans.min(cost);
    }

    println!("{ans}");
}

// alt: 構造はそのまま (累積和 + 境界全探索)。
//      - ans の初期化 + for で min 更新 → (0..=n).map(..).min() の 1 式
//      - 一度しか使わない a, b, cost の束縛を削除
// let ans = (0..=n).map(|k| w[k].max(r[n] - r[k])).min().unwrap();

// alt: 最適な境界は k = R の総数 (このとき a = b となり入れ替えだけで済む) と見抜けば累積和も
//      全探索も不要で最短・O(N)。答えは「先頭 (R の総数) 文字の中の W の数」
// let r = c.iter().filter(|&&x| x == 'R').count();
// let ans = c[..r].iter().filter(|&&x| x == 'W').count();

// alt: 操作をそのままシミュレーションする 2 ポインタ。左端から最初の W、右端から最初の R を
//      探して入れ替えるのを繰り返す。「入れ替えだけで直せる」が手順として見えるので直感的
// let mut c = c;
// let (mut i, mut j) = (0, n - 1);
// let mut ans = 0;
// loop {
//     while i < j && c[i] == 'R' { i += 1; }
//     while i < j && c[j] == 'W' { j -= 1; }
//     if i >= j { break; }
//     c.swap(i, j);
//     ans += 1;
// }
