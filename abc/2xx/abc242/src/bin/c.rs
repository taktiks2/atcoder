// algo: dp
// solve: copy 2026-10-22
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

const MOD: u64 = 998244353;

// 1〜9 を N 個並べ、隣り合う数字の差が 1 以下になる並べ方を数える問題。全部並べると 9^N 通り
// (N <= 10^6) なので、並べ方そのものを列挙するのは論外。
//
// 次の桁に置ける数字は「直前の数字」だけで決まり、それより前の桁は関係ない。だから
// 「i 桁目まで決めて最後が d の並べ方の数」= dp[d] だけ持てば、途中の並べ方を忘れてよい。
// d を置けるのは直前が d-1, d, d+1 のときなので ndp[d] = dp[d-1] + dp[d] + dp[d+1]。
// 状態 9 個 × 遷移 3 通り × N 桁 ≈ 2.7*10^7 回で間に合う。
//
// 配列を長さ 11 にして dp[0] と dp[10] を常に 0 にしておくのは番兵。1 は 0 と 2 から、
// 9 は 8 と 10 から足すことになるが、0 と 10 の分は 0 なので、端だけ遷移が 2 通りになる
// ことを if で書き分けなくて済む。
//
// 落とし穴: 0 は使えない。初期化で dp[0] = 1 にすると 0 から始まる並べ方まで数えてしまい、
// N = 2 で 25 ではなく 28 (01, 00, 10 の 3 つが余計) になる。番兵の dp[0] は 0 のまま触らない。
// 型: 遷移で足すのは MOD 未満の値 3 つで最大 ≈ 3*10^9、最後の総和は 9 つで ≈ 9*10^9。
// 後者は u32 の上限 4.29*10^9 を超えるので u64。遷移のたびに % MOD を取るので途中で溢れることはない。

#[fastout]
fn main() {
    input! {
        n: usize,
    };

    let mut dp = [0u64; 11];

    for i in 1..=9 {
        dp[i] = 1;
    }

    for _ in 1..n {
        // next dpの略
        let mut ndp = [0u64; 11];
        for i in 1..=9 {
            ndp[i] = (dp[i - 1] + dp[i] + dp[i + 1]) % MOD;
        }
        dp = ndp;
    }

    let mut ans = dp.iter().sum::<u64>() % MOD;

    println!("{ans}");
}

// alt: 構造はそのまま。冗長な部分だけ削った版
//      - 初期化の for 3 行 → 配列リテラル 1 行 (番兵の 0 が両端にあることが見た目で分かる)
//      - `let mut ans` → `let ans` (再代入しないので mut 不要。unused_mut の警告も消える)
// let mut dp = [0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0u64];
// for _ in 1..n {
//     let mut ndp = [0u64; 11];
//     for i in 1..=9 {
//         ndp[i] = (dp[i - 1] + dp[i] + dp[i + 1]) % MOD;
//     }
//     dp = ndp;
// }
// let ans = dp.iter().sum::<u64>() % MOD;

// alt: 計算量 O(9^3 log N)。遷移が桁によらず同じなので、9×9 行列 A (|i-j| <= 1 なら 1) の
//      N-1 乗として繰り返し二乗法で求められる。N が 10^18 でも解ける (本問の N <= 10^6 では不要)。
//      全要素 1 のベクトルに A^(N-1) を掛けて総和を取る = A^(N-1) の全要素の和
// let mul = |a: &[[u64; 9]; 9], b: &[[u64; 9]; 9]| {
//     let mut c = [[0u64; 9]; 9];
//     for i in 0..9 {
//         for k in 0..9 {
//             for j in 0..9 {
//                 c[i][j] = (c[i][j] + a[i][k] * b[k][j]) % MOD;
//             }
//         }
//     }
//     c
// };
// let mut a = [[0u64; 9]; 9];
// for i in 0..9 {
//     for j in 0..9 {
//         if (i as i64 - j as i64).abs() <= 1 {
//             a[i][j] = 1;
//         }
//     }
// }
// let mut r = [[0u64; 9]; 9]; // 単位行列
// for i in 0..9 {
//     r[i][i] = 1;
// }
// let mut e = n - 1;
// while e > 0 {
//     if e & 1 == 1 {
//         r = mul(&r, &a);
//     }
//     a = mul(&a, &a);
//     e >>= 1;
// }
// let ans = r.iter().flatten().sum::<u64>() % MOD;
