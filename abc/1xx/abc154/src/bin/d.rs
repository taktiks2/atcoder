// algo: math, prefix-sum
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

// 連続する K 個のサイコロを選び、出目の合計の期待値を最大化する問題。
//
// 鍵は期待値の線形性。合計の期待値は各サイコロの期待値の和で、目 1..=p が等確率なら
// 期待値は (1 + 2 + ... + p) / p = (p + 1) / 2。よって問題は「長さ K の区間で
// e_i = (p_i + 1) / 2 の和を最大化する」に落ち、確率の話は最初の一手で消える。
//
// 区間ごとに K 個足し直すと O(NK) で、N = K/2 = 10^5 付近で 10^10 規模になり TLE。
// 累積和 expects[i] = e_0 + ... + e_{i-1} を持てば区間和は expects[i + k] - expects[i]
// の O(1) で引けるので、全体 O(N)。i の範囲は 0..=(n - k) で、K <= N の制約から
// n - k は usize でもアンダーフローしない。
//
// 各 e_i を (1..=pi).sum() で作っているのは O(Σp_i) = 最大 2×10^5 × 1000 = 2×10^8 回の
// 加算になる。release ビルドなら通るが、(p + 1) / 2 の閉じた式にすれば O(1) で済む
// (下の alt 参照)。
//
// 精度: 累積和の最大値は 2×10^5 × 500.5 ≈ 10^8 で、f64 の相対誤差 2.2×10^-16 を掛けても
// 絶対誤差は 10^-8 程度。許容誤差 10^-6 に十分収まる。そもそも Σp_i は整数なので、
// 整数のまま最大区間を求めて最後に 1 回だけ割れば誤差はゼロになる (下の alt 参照)。
//
// max の初期値 0 は、期待値が必ず 1 以上なので必ず上書きされる。
//
// 落とし穴は出力形式。println!("{max}") は 7.0 を "7" と出すので、cargo compete test の
// 既定 (文字列の完全一致) では "7.000000000000" と食い違い、全サンプル WA に見える。
// AtCoder のジャッジは誤差判定なので "7" でも AC。ローカルでは testcases/d.yml の
// match を Float にして合わせた。出力は 1 行なので #[fastout] は本問では効いていない。

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        p: [usize; n],
    };

    let mut expects: Vec<f64> = vec![0f64; n + 1];
    let mut total = 0f64;

    for (i, &pi) in p.iter().enumerate() {
        total += (1..=pi).sum::<usize>() as f64 / pi as f64;
        expects[i + 1] = total;
    }

    let mut max = 0f64;

    for i in 0..=(n - k) {
        max = max.max(expects[i + k] - expects[i]);
    }

    println!("{max}");
}

// alt: 構造はそのまま (累積和配列 + 区間差の max ループ)。冗長な部分だけ削った版
//      - (1..=pi).sum() / pi → (pi + 1) / 2 の閉じた式。O(p) が O(1) になる
//      - 別変数 total を消し、expects[i] から直接 expects[i + 1] を作る
// let mut expects = vec![0f64; n + 1];
// for (i, &pi) in p.iter().enumerate() {
//     expects[i + 1] = expects[i] + (pi + 1) as f64 / 2.0;
// }
// let mut max = 0f64;
// for i in 0..=(n - k) {
//     max = max.max(expects[i + k] - expects[i]);
// }
// println!("{max}");

// alt: 整数のスライディングウィンドウ。Σ(p_i + 1) / 2 = (Σp_i + K) / 2 なので、
//      p の区間和の最大値だけを usize で求め、最後に 1 回だけ f64 にする。浮動小数の誤差が
//      原理的に出ず、累積和配列も要らないのでメモリは O(1)。sum + p[i] - p[i - k] は
//      足してから引く順なので usize でもアンダーフローしない。{:.12} にすれば出力も
//      サンプルと文字列一致する
// let mut sum: usize = p[..k].iter().sum();
// let mut best = sum;
// for i in k..n {
//     sum = sum + p[i] - p[i - k];
//     best = best.max(sum);
// }
// println!("{:.12}", (best + k) as f64 / 2.0);
