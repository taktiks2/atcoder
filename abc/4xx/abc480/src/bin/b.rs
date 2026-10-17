// algo: brute-force, string
// solve: self 2026-10-17
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

// 各投票用紙 T_j を「完全一致 → 1 文字違いがちょうど 1 人 → 無効票」の優先順で判定する問題。
// N, M <= 100、|S_i|, |T_j| <= 10 なので、全ペアを 1 文字ずつ比べても 100 * 100 * 10 = 10^5 回。
// 工夫は要らず、規則をそのまま書く全探索でよい。
//
// 「1 文字書き換えて一致」は「長さが同じで、異なる位置がちょうど 1 か所」と同値（挿入・削除は
// 含まない）。だから長さが違う候補は最初に弾ける。この長さチェックは省けない: zip は短い方で
// 止まるので、外すと "a" と "aa" が「違う位置 0 か所」= 完全一致扱いになる（サンプル 2 の "aa"）。
//
// 規則 1 は規則 2 より優先される。完全一致がある用紙は、1 文字違いの候補が他に何人いても
// 完全一致の人に入る（サンプル 1 の "kato" は sato, kano とも 1 文字違い）。だから走査中は
// comp と near を別々に記録し、ループを抜けてから comp を先に見る。near を見つけた瞬間に
// 投票するとこの優先順が崩れる。S_i は互いに異なるので完全一致は高々 1 人で、comp_idx が
// 上書きされる心配はない。
//
// 空の `else if near > 1 {}` は「1 文字違いが複数 = 無効票」を明示しているだけで何もしない。
// 規則 2 の「ちょうど 1 人」を落として near >= 1 で投票すると、最後に見た near_idx に票が
// 入って WA（サンプル 1 の "sano"）。

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        s: [String; n],
        t: [String; m],
    };

    let mut votes: Vec<usize> = vec![0; n];

    for ti in &t {
        let mut comp = false;
        let mut near = 0usize;
        let mut comp_idx = 0usize;
        let mut near_idx = 0usize;
        for (i, si) in s.iter().enumerate() {
            if si.len() == ti.len() {
                let mut ng = 0usize;
                for (a, b) in si.chars().zip(ti.chars()) {
                    if a != b {
                        ng += 1;
                    }
                }
                match ng {
                    0 => {
                        comp = true;
                        comp_idx = i;
                    }
                    1 => {
                        near += 1;
                        near_idx = i;
                    }
                    _ => {}
                }
            }
        }
        if comp {
            votes[comp_idx] += 1;
        } else if near > 1 {
        } else if near == 1 {
            votes[near_idx] += 1;
        }
    }

    for v in votes {
        println!("{v}");
    }
}

// alt: 構造はそのまま（候補ごとに違う文字数 ng を数え、comp / near を記録してから判定）。
//      冗長な部分だけ削った版
//      - 長さ一致の if ネスト → `!=` で continue して 1 段浅く
//      - ng を数える for + if → zip().filter().count() の 1 行
//      - 空の `else if near > 1 {}` → 削除（near == 1 だけ見れば同じ）
//      - `Vec<usize>` の型注釈 → vec![0usize; n] に寄せる
// let mut votes = vec![0usize; n];
// for ti in &t {
//     let mut comp = false;
//     let mut near = 0usize;
//     let mut comp_idx = 0usize;
//     let mut near_idx = 0usize;
//     for (i, si) in s.iter().enumerate() {
//         if si.len() != ti.len() {
//             continue;
//         }
//         let ng = si.chars().zip(ti.chars()).filter(|(a, b)| a != b).count();
//         match ng {
//             0 => {
//                 comp = true;
//                 comp_idx = i;
//             }
//             1 => {
//                 near += 1;
//                 near_idx = i;
//             }
//             _ => {}
//         }
//     }
//     if comp {
//         votes[comp_idx] += 1;
//     } else if near == 1 {
//         votes[near_idx] += 1;
//     }
// }
// for v in votes {
//     println!("{v}");
// }

// alt: 規則の優先順をそのままコードの順序にした版（Rust らしさ）。完全一致を position で先に
//      探して continue し、残りだけ 1 文字違いを集める。comp / near のフラグと添字の 4 変数が
//      消え、`if let [i] = near[..]` で「ちょうど 1 人」をスライスパターンとして書ける
// let mut votes = vec![0usize; n];
// for ti in &t {
//     if let Some(i) = s.iter().position(|si| si == ti) {
//         votes[i] += 1;
//         continue;
//     }
//     let near: Vec<usize> = (0..n)
//         .filter(|&i| {
//             s[i].len() == ti.len()
//                 && s[i].bytes().zip(ti.bytes()).filter(|(a, b)| a != b).count() == 1
//         })
//         .collect();
//     if let [i] = near[..] {
//         votes[i] += 1;
//     }
// }
// for v in votes {
//     println!("{v}");
// }

// alt: 名前 -> 添字の HashMap を作り、T_j の 1 文字違いを 25 * |T_j| 通り生成して引く版（計算量）。
//      O(M * 26 * |T|^2) で N に依存しないので、N が 10^5 規模でも通る。生成する文字列は
//      互いに異なるので hit に重複は入らず、hit が 1 個なら「ちょうど 1 人」
// let idx: HashMap<&[u8], usize> = s.iter().enumerate().map(|(i, x)| (x.as_bytes(), i)).collect();
// let mut votes = vec![0usize; n];
// for ti in &t {
//     if let Some(&i) = idx.get(ti.as_bytes()) {
//         votes[i] += 1;
//         continue;
//     }
//     let mut b = ti.clone().into_bytes();
//     let mut hit = vec![];
//     for p in 0..b.len() {
//         let orig = b[p];
//         for c in b'a'..=b'z' {
//             if c == orig {
//                 continue;
//             }
//             b[p] = c;
//             if let Some(&i) = idx.get(&b[..]) {
//                 hit.push(i);
//             }
//         }
//         b[p] = orig;
//     }
//     if let [i] = hit[..] {
//         votes[i] += 1;
//     }
// }
// for v in votes {
//     println!("{v}");
// }
