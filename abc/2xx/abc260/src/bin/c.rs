// algo: simulation
// solve: hint 2026-10-19
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

// レベル N の赤 1 個から始め、変換を繰り返してレベル 1 の青を最大化する問題。
// どちらの変換も宝石を減らさず、レベル 1 の青を増やす方向にしか働かない。なので「変換できる
// ものは全部、レベル 1 まで変換し切る」のが最適で、選択の余地は無い。状態ごとに選択肢を比べる
// DP ではなく、全部の変換を個数ぶんまとめて実行する一括シミュレーション。各レベルの赤と青の
// 個数を持って、上のレベルから順に下へ流すだけ。
//
// 処理順はレベルの高い方から。レベル L の青は、レベル L の赤の変換でしか生まれない。だから
// 「L の赤 → L の青」を先に済ませ、その後に「L の青 → L-1」を流せば、各レベルを 1 回ずつ
// 処理するだけで取りこぼしが無い。r[i] / b[i] はレベル i+1 の個数（0-indexed）。
//
// 落とし穴: 変換は 1 個ずつではなく、個数ぶんまとめて流す。r[L-1] += 1 と書くと、赤が 1 個
// しかない上位 2 レベル（N=2 の Sample 1 はこれで全部）ではたまたま合い、赤が 2 個以上に
// なる下位レベルからずれる。それで Sample 3 だけ WA になる。
//
// 型: 最大ケース N=10, X=Y=5 で答えは 3942349900 ≈ 3.9*10^9。i32 の 2.1*10^9 も
// u32 の 4.29*10^9 もぎりぎりなので 64 bit が要る（usize は 64 bit 環境なので可）。
// 計算量は O(N)。

#[fastout]
fn main() {
    input! {
        n: usize,
        x: usize,
        y: usize,
    };

    let mut r: Vec<usize> = vec![0; n];
    let mut b: Vec<usize> = vec![0; n];

    let mut last_r = n - 1;
    let mut last_b = n - 1;

    r[last_r] = 1;

    while last_r > 0 && last_b > 0 {
        if last_r > 0 {
            let r_count = r[last_r];
            r[last_r - 1] += r_count;
            b[last_r] += x * r_count;
            last_r -= 1;
        }

        if last_b > 0 {
            let b_count = b[last_b];
            r[last_b - 1] += b_count;
            b[last_b - 1] += y * b_count;
            last_b -= 1;
        }
    }

    println!("{}", b[0]);
}

// alt: 構造はそのまま（Vec 2 本で上のレベルから流す）。冗長な部分だけ削った版
//      - last_r / last_b は常に同じ値で一緒に減るので、ループ変数 i 1 本にまとめる
//      - while + 内側の if 2 本 → for i in (1..n).rev()（条件が常に真の if が消える）
//      - r_count / b_count の一時変数 → r[i] / b[i] を直接読む
// let mut r: Vec<usize> = vec![0; n];
// let mut b: Vec<usize> = vec![0; n];
// r[n - 1] = 1;
// for i in (1..n).rev() {
//     r[i - 1] += r[i];
//     b[i] += x * r[i];
//     r[i - 1] += b[i];
//     b[i - 1] += y * b[i];
// }
// println!("{}", b[0]);

// alt: 配列が要らない版。使うのは常に「今のレベル」の赤と青だけなので、スカラー 2 個で済む。
//      1 段下ろすたびに、赤 r 個 → 青 +x*r、青 b 個 → 赤 +b・青 b*y。メモリ O(1)・最短
//      (入力は x: u64, y: u64 で読む)
// let (mut r, mut b) = (1u64, 0u64);
// for _ in 1..n {
//     b += x * r; // レベル L の赤を変換 → レベル L の青が増える
//     r += b;     // レベル L の青を変換 → レベル L-1 の赤（赤由来 r 個 + 青由来 b 個）
//     b *= y;     // 同じく → レベル L-1 の青
// }
// println!("{b}");
