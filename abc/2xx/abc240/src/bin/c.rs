// algo: dp
// solve: copy 2026-10-23
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

// 各ジャンプで a_i か b_i を選び、合計をちょうど X にできるかを判定する問題。
// 選び方は 2^N 通りで、N <= 100 なら 2^100。全部試すのは論外。
//
// ただし、選び方が違っても同じ座標にたどり着くなら、その先の可能性はまったく同じになる。
// 区別すべきなのは「どう選んだか」ではなく「今どの座標にいるか」だけ。
// 座標は高々 100 * 100 = 10^4 通りしかないので、dp[p] = 「今の回数で座標 p にいられるか」を
// bool で持ち、1 回ごとに a_i, b_i の 2 通りの遷移を配る。O(N X) ≈ 10^6 で十分間に合う。
//
// 進む量は必ず正なので、X を超えた座標が X に戻ることはない。配列は 0..=X だけ持てばよく、
// p + a > X の遷移は捨ててよい。
//
// 落とし穴 1: 「ちょうど N 回」跳ぶので、毎回 next を作り直してから dp に差し替える。
//   dp を直接書き換えると、同じ回のうちに更新済みのマスから
//   さらに跳んでしまい、1 回で 2 回分進んだ状態が混ざる。
// 落とし穴 2: dp の長さは X + 1 で、N + 1 ではない。添字は回数ではなく座標。
//   長さを N + 1 にすると、X > N のときに p = N + 1 で範囲外アクセスになる。
//   （サンプル 1 は N = 2, X = 10）

#[fastout]
fn main() {
    input! {
        n: usize,
        x: usize,
        ab: [(usize, usize); n],
    };

    let mut dp = vec![false; x + 1];

    dp[0] = true;

    for &(a, b) in &ab {
        let mut next = vec![false; x + 1];
        for p in 0..=x {
            if !dp[p] {
                continue;
            }
            if p + a <= x {
                next[p + a] = true;
            }
            if p + b <= x {
                next[p + b] = true;
            }
        }
        dp = next;
    }

    println!("{}", if dp[x] { "Yes" } else { "No" });
}

// alt: 構造はそのまま (dp / next の持ち回し)。冗長な分岐を削った版
//      - `if !dp[p] { continue; }` → `filter(|&p| dp[p])`
//      - 同じ形の if 2 本 → `for q in [p + a, p + b]` の 1 本
// let mut dp = vec![false; x + 1];
// dp[0] = true;
// for &(a, b) in &ab {
//     let mut next = vec![false; x + 1];
//     for p in (0..=x).filter(|&p| dp[p]) {
//         for q in [p + a, p + b] {
//             if q <= x {
//                 next[q] = true;
//             }
//         }
//     }
//     dp = next;
// }
// println!("{}", if dp[x] { "Yes" } else { "No" });

// alt: いられる座標の集合をそのまま持ち回す版。配列の上限を考えなくてよく、
//      「到達できる座標の集合」という意味が型に出る。集合の要素数も 10^4 以下なので
//      計算量は同じで、遅くなるのは HashSet の定数倍だけ
// let mut cur: HashSet<usize> = HashSet::from([0]);
// for &(a, b) in &ab {
//     cur = cur.iter().flat_map(|&p| [p + a, p + b]).filter(|&q| q <= x).collect();
// }
// println!("{}", if cur.contains(&x) { "Yes" } else { "No" });
