// algo: dp
// solve: copy 2026-10-21
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

const MOD: u64 = 998244353;

// 1〜M の整数を N 個並べ、合計が K 以下になる列の個数を数える問題。
// 全列挙は M^N = 50^50 通りで論外。列そのものではなく「合計ごとの個数」を持ち運ぶ。
//
// dp[i][s] = i 個目まで決めて合計がちょうど s になる列の個数。
// 長さ i+1 で合計 t の列は、最後の値 a (1〜M) で分けると「長さ i・合計 t-a の列 + a」に
// ちょうど 1 通りに分解できる (最後の値は必ずどれか 1 つ → 漏れなし、a か前半が違えば
// 別の列 → ダブりなし)。後ろに付けられる値は合計だけで決まり、途中の並びには制約が
// 無いので、中身の違う列を合計ごとに束ねて個数だけ持てばよい。
//
// dp[0][0] = 1 は「空の列 () が 1 個」。これを置くと 1 行目も dp[0] からの同じ遷移で
// 作れ、初期化を特別扱いしなくて済む。0 にすると全部 0、2 にすると全部 2 倍になる。
//
// 合計が K を超えた状態は答えに効かないので s の上限は K で打ち切る。
// 状態 (N+1)(K+1) ≈ 51·2501、各 M 通りの遷移で O(NKM) ≈ 6.25·10^6。
//
// 型は u64。各値は MOD 未満なので 2 つの和は 2·10^9 程度で溢れない。最後の合計も
// 2501 個 × 10^9 ≈ 2.5·10^12 で u64 に収まるので、足してから 1 回割れば足りる。
//
// 落とし穴:
// - 答えは dp[N][K] ではなく dp[N][0..=K] の合計 (条件が「ちょうど K」ではなく「K 以下」)。
//   サンプル 1 で dp[2][4] = 3 だけを出すと WA
// - 「s から s+a に配る」向きで書くと usize の s-a で負になる心配がない。
//   「t-a から集める」向きだと a > t で usize がパニックする

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
    };

    let mut dp = vec![vec![0u64; k + 1]; n + 1];
    dp[0][0] = 1;
    for i in 0..n {
        for s in 0..=k {
            if dp[i][s] == 0 {
                continue;
            }
            for a in 1..=m {
                if s + a > k {
                    break;
                }
                dp[i + 1][s + a] = (dp[i + 1][s + a] + dp[i][s]) % MOD;
            }
        }
    }

    let mut ans = dp[n].iter().sum::<u64>() % MOD;

    println!("{ans}");
}

// alt: 構造はそのまま (2 次元 dp・配る向きの 3 重ループ) で、冗長な分岐を削った版
// - `if dp[i][s] == 0 { continue; }` を削除: 0 を足しても結果は同じで、O(NKM) も変わらない
// - `if s + a > k { break; }` を範囲 `1..=m.min(k - s)` に畳む (s <= k なので k - s は負にならない)
// - `let mut ans` → `let ans` (再代入していない)
// let mut dp = vec![vec![0u64; k + 1]; n + 1];
// dp[0][0] = 1;
// for i in 0..n {
//     for s in 0..=k {
//         for a in 1..=m.min(k - s) {
//             dp[i + 1][s + a] = (dp[i + 1][s + a] + dp[i][s]) % MOD;
//         }
//     }
// }
// let ans = dp[n].iter().sum::<u64>() % MOD;

// alt: 累積和で遷移を O(1) にして O(NK) に落とす (M や K がもっと大きいとき用)。
//      dp[i+1][t] = dp[i][t-M] + ... + dp[i][t-1] は dp[i] の区間和なので、
//      累積和 acc の差 acc[t] - acc[max(t-M, 0)] で出る。行は 1 本ずつ持てば足りる。
//      引き算は MOD を足してから割らないと u64 が負に回り込む
// let mut cur = vec![0u64; k + 1];
// cur[0] = 1;
// for _ in 0..n {
//     let mut acc = vec![0u64; k + 2];
//     for s in 0..=k {
//         acc[s + 1] = (acc[s] + cur[s]) % MOD;
//     }
//     let mut nxt = vec![0u64; k + 1];
//     for t in 1..=k {
//         let lo = t.saturating_sub(m);
//         nxt[t] = (acc[t] + MOD - acc[lo]) % MOD;
//     }
//     cur = nxt;
// }
// let ans = cur.iter().sum::<u64>() % MOD;
