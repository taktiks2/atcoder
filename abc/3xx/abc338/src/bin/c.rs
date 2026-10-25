// algo: brute-force
// solve: hint 2026-10-25
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

// A を x 人分、B を y 人分作るときの x + y を最大化する問題。x と y を両方回すと
// それぞれ 10^6 通りで 10^12 になり TLE。
//
// x を決めると y の上限は一意に決まる。材料 i の残りは Q_i - x*A_i で、B を作るには
// 全材料で y*B_i <= 残り が要るので、y = min_{B_i > 0} (Q_i - x*A_i) / B_i (切り捨て)。
// B_i = 0 の材料は y を縛らないので除外する (割ると 0 除算)。B_i >= 1 の材料が必ず
// 一つある制約のおかげで、x が作れる限り min は必ず上書きされ i64::MAX は残らない。
//
// x の上限は A_i >= 1 の材料が必ずあることから出る。Q_i <= 10^6, A_i >= 1 なので
// x <= 10^6。よって x を 0..=10^6 で回し、各 x で N <= 10 個を見るので 10^7 回で間に合う。
//
// 型は i64。ai * i は最大 10^6 * 10^6 = 10^12 で i32 の 2.1*10^9 を超える。また
// sub が負になりうる (= その x は作れない) ので符号付きが必要。
//
// 落とし穴 (どちらもサンプルは通るのに WA になった形):
// - 範囲を `0..1_000_000` にすると x = 10^6 ちょうどを試さない。N=1, Q=10^6, A=1,
//   B=10^6 で正解 10^6 に対し 999999 を出す。上限を含める `..=` が必須
// - 判定を `sub > 0` にすると、材料をちょうど使い切る x を作れない扱いにしてしまう。
//   N=2, Q=(1,10), A=(1,0), B=(0,10) で正解 2 (A=1, B=1) に対し 1 を出す
//
// 「A を作れるだけ作って残りで B」の貪欲は誤り。サンプル 1 で x = 3 にすると合計 3 だが、
// 最適は x = 2, y = 3 の 5。x を全部試す必要がある。
// 出力は 1 行なので #[fastout] は本問では効いていない。

#[fastout]
fn main() {
    input! {
        n: usize,
        q: [i64; n],
        a: [i64; n],
        b: [i64; n],
    };

    let mut ans = 0i64;
    for i in 0..=1_000_000 {
        let mut ok = true;
        let mut min = i64::MAX;
        for ((&qi, &ai), &bi) in q.iter().zip(&a).zip(&b) {
            let sub = qi - ai * i;
            if sub >= 0 {
                if bi > 0 {
                    let b_cnt = sub / bi;
                    min = min.min(b_cnt);
                }
            } else {
                ok = false;
                break;
            }
        }

        if ok {
            ans = ans.max(i + min);
        }
    }

    println!("{ans}");
}

// alt: 構造はそのまま (x の全探索 + 材料ごとの min)。冗長な部分だけ削った版
//      - ok フラグ → ラベル付き break 'outer。A_i >= 0 なので一度足りなくなった材料は
//        x を増やしても足りないまま。以降の x は全部作れないので、continue ではなく
//        外側ごと抜けてよい (無駄な残り最大 10^6 周も消える)
//      - if sub >= 0 { if bi > 0 {..} } else {..} の入れ子 → 早期脱出で平たく
//      - b_cnt の一時変数を消して min.min(sub / bi) に直接
// let mut ans = 0i64;
// 'outer: for i in 0..=1_000_000 {
//     let mut min = i64::MAX;
//     for ((&qi, &ai), &bi) in q.iter().zip(&a).zip(&b) {
//         let sub = qi - ai * i;
//         if sub < 0 {
//             break 'outer;
//         }
//         if bi > 0 {
//             min = min.min(sub / bi);
//         }
//     }
//     ans = ans.max(i + min);
// }
// println!("{ans}");

// alt: x の上限を先に min_{A_i > 0} Q_i / A_i で求めておけば、範囲内の x は全材料で
//      sub >= 0 が保証されるので可否判定そのものが消える。10^6 という定数を書かずに
//      済むので、本番で踏んだ `..` と `..=` の取り違えや sub > 0 の境界ミスが構造的に
//      起きない。filter + min / max のイテレータで式一つ。unwrap は「A_i >= 1, B_i >= 1
//      の材料が必ずある」制約から安全
// let x_max = (0..n).filter(|&i| a[i] > 0).map(|i| q[i] / a[i]).min().unwrap();
// let ans = (0..=x_max)
//     .map(|x| {
//         x + (0..n)
//             .filter(|&i| b[i] > 0)
//             .map(|i| (q[i] - a[i] * x) / b[i])
//             .min()
//             .unwrap()
//     })
//     .max()
//     .unwrap();
// println!("{ans}");
