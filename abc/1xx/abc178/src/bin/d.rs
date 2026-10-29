// algo: dp
// solve: copy 2026-10-29
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

// 各項 3 以上・総和 S の数列を数える。順番違いは別物として数える。
// 長さが決まっていないので「長さ k を全探索して各項を割り振る」より、
// 総和で状態を切る DP の方が素直。
//
// dp[i] = 総和 i になる数列の個数。最後の項 x (3..=i) で場合分けすると、手前の部分は
// 総和 i - x の数列で、x の選び方ごとに重複なく分かれる。よって
//   dp[i] = Σ_{x=3}^{i} dp[i - x] = dp[0] + dp[1] + ... + dp[i - 3]
// i の小さい順に埋めれば、右辺は必ず計算済み。
//
// dp[0] = 1 は「空の数列」。これが無いと x = i (項が {i} 一つだけ) の 1 通りが
// 数えられず、全部 0 になる。S = 7 なら {3,4}, {4,3}, {7} の {7} がこの寄与。
// S < 3 のときは内側ループが一度も回らず dp[S] = 0 のままなので特別扱いは要らない。
//
// 計算量は Σ_{i} (i - 2) ≈ S^2 / 2 = 2×10^6 回 (S <= 2000) で余裕。
// 型は u64。足すたびに % MOD するので値は常に MOD (≈ 10^9) 未満で、
// 1 回の足し算は 2×10^9 程度と u64 (≈ 1.8×10^19) に遠く及ばない。
//
// 落とし穴は内側ループの範囲。3..i (i を含まない) にすると上と同じく {i} 単独が抜け、
// S = 7 で 2 になる。
const MOD: u64 = 1_000_000_007;

#[fastout]
fn main() {
    input! {
        s: usize,
    };

    let mut dp = vec![0u64; s + 1];
    dp[0] = 1;
    for i in 1..=s {
        for x in 3..=i {
            dp[i] = (dp[i] + dp[i - x]) % MOD;
        }
    }

    println!("{}", dp[s]);
}

// alt: 構造はそのまま (dp[i] = dp[0..=i-3] の和 / O(S^2))。内側ループを sum に置き換えた
//      - 二重ループ + 毎回 % → スライスの iter().sum() 1 行
//      - i < 3 は dp[i] = 0 のままでよいので外側ループを 3 から始めて i - 3 のアンダーフローを回避
//      - 和は最大 2000 × 10^9 = 2×10^12 なので u64 のまま足してから最後に % 1 回で足りる
// let mut dp = vec![0u64; s + 1];
// dp[0] = 1;
// for i in 3..=s {
//     dp[i] = dp[..=i - 3].iter().sum::<u64>() % MOD;
// }
// println!("{}", dp[s]);

// alt: 計算量 O(S)。dp[i] と dp[i-1] の和は dp[i-3] の 1 項しか違わないので
//      dp[i] = dp[i-1] + dp[i-3] と書き直せ、内側ループが消える (累積和を持つのと同じ発想)
// let mut dp = vec![0u64; s + 1];
// dp[0] = 1;
// for i in 3..=s {
//     dp[i] = (dp[i - 1] + dp[i - 3]) % MOD;
// }
// println!("{}", dp[s]);

// alt: DP を使わず組合せで直接数える。項数 k を固定し各項から 2 を引くと
//      「正の整数 k 個で総和 S - 2k」になり、仕切り板で C(S - 2k - 1, k - 1) 通り。
//      k = 1..=S/3 で足す。階乗と逆元 (フェルマー) の前計算が要るので DP より手間は多いが、
//      S が 10^6 級でも同じ形で通る
// let mut fact = vec![1u64; s + 1];
// for i in 1..=s { fact[i] = fact[i - 1] * i as u64 % MOD; }
// let pow = |mut b: u64, mut e: u64| { let mut r = 1; while e > 0 { if e & 1 == 1 { r = r * b % MOD; } b = b * b % MOD; e >>= 1; } r };
// let mut inv = vec![1u64; s + 1];
// inv[s] = pow(fact[s], MOD - 2);
// for i in (1..=s).rev() { inv[i - 1] = inv[i] * i as u64 % MOD; }
// let comb = |n: usize, r: usize| fact[n] * inv[r] % MOD * inv[n - r] % MOD;
// let ans = (1..=s / 3).map(|k| comb(s - 2 * k - 1, k - 1)).fold(0, |acc, c| (acc + c) % MOD);
// println!("{ans}");
