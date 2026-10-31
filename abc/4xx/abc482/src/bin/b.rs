// algo: grid, brute-force
// solve: self 2026-10-31
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

// 5x5 のグリッドで、あと何マス埋めればビンゴ (縦・横・斜めのどれか 1 列が全部 'o')
// になるかの最小値を答える問題。
//
// ある 1 列を完成させるのに必要なのは、その列の '.' を全部埋めることで、それ以上も
// それ以下も要らない。列どうしは独立に考えてよい (1 列揃えば終わりなので、
// 複数の列にまたがる得を狙う必要が無い)。よって答えは「12 本の列それぞれの '.' の数」の
// 最小値。候補は横 5 + 縦 5 + 斜め 2 = 12 本 × 5 マスで、全部数えても 60 回。
//
// 落とし穴は斜めの数え漏れ。横と縦だけでもサンプル 1・2 は通るが、斜めが最小になる
// ケース (サンプル 3 の左上→右下は残り 1) で WA になる。逆対角は (i, 4 - i)。
// cnt の初期値 usize::MAX は「まだ候補を見ていない」ための番兵で、縦のループで
// 必ず 5 本ぶん更新される。

#[fastout]
fn main() {
    input! {
        s: [Chars; 5],
    };

    // 横
    let row_cnt = s
        .iter()
        .map(|arr| arr.iter().filter(|&&c| c == '.').count())
        .min()
        .unwrap();

    let mut cnt = usize::MAX;
    for i in 0..5usize {
        // 縦
        let mut col = 0usize;
        for si in &s {
            if si[i] == '.' {
                col += 1;
            }
        }
        cnt = cnt.min(col);
    }
    // 斜め
    let mut cross1 = 0usize;
    let mut cross2 = 0usize;
    for i in 0..5usize {
        if s[i][i] == '.' {
            cross1 += 1;
        }

        if s[i][4 - i] == '.' {
            cross2 += 1;
        }
    }

    cnt = row_cnt.min(cnt.min(cross1).min(cross2));

    println!("{cnt}");
}

// alt: 構造はそのまま (横・縦・斜め 2 本を別々に数えて最後に min) で、冗長な部分だけ削った版。
//      - 縦のループ + 可変カウンタ → 横と同じ map + filter + count + min の形に揃えた
//      - cnt の番兵 usize::MAX が不要になった (min を iterator 側で取るため)
//      - 斜めの if 2 本 → filter + count 2 行
// let row = s.iter().map(|r| r.iter().filter(|&&c| c == '.').count()).min().unwrap();
// let col = (0..5).map(|j| s.iter().filter(|r| r[j] == '.').count()).min().unwrap();
// let cross1 = (0..5).filter(|&i| s[i][i] == '.').count();
// let cross2 = (0..5).filter(|&i| s[i][4 - i] == '.').count();
// let cnt = row.min(col).min(cross1).min(cross2);
// println!("{cnt}");

// alt: 12 本の列を座標リストとして先に作り、数える処理を 1 か所にまとめる版。
//      「どの列を見るか」と「列の '.' を数える」が分離されるので、斜めの書き漏れや
//      縦横の添字取り違えが起きにくい。N×N や「ビンゴの判定」など列を使い回す問題にもそのまま広がる
// let mut lines: Vec<Vec<(usize, usize)>> = vec![];
// for i in 0..5 {
//     lines.push((0..5).map(|j| (i, j)).collect()); // 横
//     lines.push((0..5).map(|j| (j, i)).collect()); // 縦
// }
// lines.push((0..5).map(|i| (i, i)).collect()); // 左上→右下
// lines.push((0..5).map(|i| (i, 4 - i)).collect()); // 右上→左下
// let ans = lines
//     .iter()
//     .map(|l| l.iter().filter(|&&(i, j)| s[i][j] == '.').count())
//     .min()
//     .unwrap();
// println!("{ans}");
