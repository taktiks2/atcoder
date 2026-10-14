// algo: brute-force
// solve: self 2026-10-14
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

// 3×3 のマスを正の整数で埋め、行和 h1..h3・列和 w1..w3 に一致させる方法を数える問題。
// 9 マスを独立に 1..=28 で回すと 28^9 ≈ 10^13 で論外。行和の制約を先に使って探索空間を削る。
//
// 1 行は左 2 マス (i, j) を決めれば右端が h - i - j に決まるので、行ごとの候補は
// 「i, j >= 1 かつ h - i - j >= 1」を満たす組 = C(h-1, 2) 個。h <= 30 なら最大 C(29, 2) = 406。
// 3 行の組合せは 406^3 ≈ 6.7×10^7 で、各組は足し算と比較だけなので間に合う
// (h = w = 30 の最悪ケースで手元 release 0.1 s)。
//
// ループ上限 1..=28 / 1..=(29 - i) は h の最大値 30 から決めた固定値で、実際の範囲は
// i + j < h の判定で切っている。この判定は「右端が 1 以上」と同時に、usize の h - i - j が
// アンダーフローしない保証も兼ねている。<= にすると右端 0 を数えて WA (正の整数が条件)。
//
// 列の判定は 3 本とも必要。行和を満たした時点で全マスの和は h1+h2+h3 なので、列 1, 2 が
// 合えば列 3 は (h1+h2+h3) - w1 - w2 に自動で決まるが、それが w3 と一致するのは
// h の総和 = w の総和 のときだけ。col3 の比較を省くと総和の合わない入力で過大に数える。
// しかもサンプル 1, 3, 4 は総和が一致し、サンプル 2 は w1 + w2 > h の総和で元々 0 なので、
// 省いてもサンプルは全部通る。「サンプルは通るのに WA」の典型形。
//
// 値は最大 30、列和も 90 程度なので型は usize で十分。出力は 1 行なので #[fastout] は
// 本問では効いていない。

#[fastout]
fn main() {
    input! {
        hw: [usize; 6],
    };

    let mut arr: Vec<Vec<Vec<usize>>> = vec![vec![]; 3];

    for (index, &h) in hw[..3].iter().enumerate() {
        for i in 1..=28 {
            for j in 1..=(29 - i) {
                if i + j < h {
                    arr[index].push(vec![i, j, h - i - j]);
                }
            }
        }
    }

    let mut ans = 0usize;
    for row1 in &arr[0] {
        for row2 in &arr[1] {
            for row3 in &arr[2] {
                let col1 = row1[0] + row2[0] + row3[0];
                let col2 = row1[1] + row2[1] + row3[1];
                let col3 = row1[2] + row2[2] + row3[2];

                if col1 == hw[3] && col2 == hw[4] && col3 == hw[5] {
                    ans += 1;
                }
            }
        }
    }

    println!("{ans}");
}

// alt: 構造はそのまま (行ごとの候補列挙 → 3 重ループで列判定)。削ったのは以下
//      - 候補の型 Vec<usize> → [usize; 3]。長さ 3 固定なので配列で足り、候補ごとのヒープ確保が消える
//      - 上限 1..=28 / 1..=(29 - i) + if i + j < h → 1..h / 1..(h - i)。範囲そのものを
//        「右端 >= 1」にすれば if が消え、30 という制約値への依存も消える
//      - col1..col3 の 3 変数 + && 3 本 → (0..3).all(..) 1 行
// let mut arr: Vec<Vec<[usize; 3]>> = vec![vec![]; 3];
// for (index, &h) in hw[..3].iter().enumerate() {
//     for i in 1..h {
//         for j in 1..(h - i) {
//             arr[index].push([i, j, h - i - j]);
//         }
//     }
// }
// let mut ans = 0usize;
// for row1 in &arr[0] {
//     for row2 in &arr[1] {
//         for row3 in &arr[2] {
//             if (0..3).all(|c| row1[c] + row2[c] + row3[c] == hw[3 + c]) {
//                 ans += 1;
//             }
//         }
//     }
// }

// alt: 3 重ループを itertools の iproduct! に畳む。「直積から条件を満たすものを数える」が
//      filter + count でそのまま読め、候補生成も flat_map で式になって mut が消える。Rust らしさ
// let rows: Vec<Vec<[usize; 3]>> = hw[..3]
//     .iter()
//     .map(|&h| {
//         (1..h)
//             .flat_map(|i| (1..h - i).map(move |j| [i, j, h - i - j]))
//             .collect()
//     })
//     .collect();
// let ans = iproduct!(&rows[0], &rows[1], &rows[2])
//     .filter(|(r1, r2, r3)| (0..3).all(|c| r1[c] + r2[c] + r3[c] == hw[3 + c]))
//     .count();

// alt: 自由度は 4 しかない。左上 2×2 (a b / d e) を決めれば行和から c, f、列和から g, hh が
//      決まり、右下 i は 3 行目の行和から決まる。残る条件は「全部 1 以上」と「3 列目の列和」
//      だけ。28^4 ≈ 6.1×10^5 で AC の 6.7×10^7 より 2 桁少なく、候補の Vec も要らない。
//      引き算で負になり得るので i64 にし、入力も h, w に分けて受ける
// input! { h: [i64; 3], w: [i64; 3] };
// let mut ans = 0;
// for a in 1..=28 {
//     for b in 1..=28 {
//         for d in 1..=28 {
//             for e in 1..=28 {
//                 let c = h[0] - a - b;
//                 let f = h[1] - d - e;
//                 let g = w[0] - a - d;
//                 let hh = w[1] - b - e;
//                 let i = h[2] - g - hh;
//                 if c > 0 && f > 0 && g > 0 && hh > 0 && i > 0 && c + f + i == w[2] {
//                     ans += 1;
//                 }
//             }
//         }
//     }
// }
