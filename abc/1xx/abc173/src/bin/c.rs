// algo: brute-force, grid
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

// 行と列をいくつか選んで赤く塗り、黒マスがちょうど K 個残る選び方を数える問題。
// 行の選び方は 2^H、列の選び方は 2^W で、合わせて 2^(H+W)。H, W <= 6 なので高々 2^12 = 4096 通り。
// 1 通りごとに盤面をコピーして塗って数えても O(HW) 程度なので、全体で 4096 * 36 ≈ 1.5 * 10^5 程度で済む。
// H と W が両方とも小さい制約はビット全探索の合図。
//
// 行と列は独立に選べる。塗る操作は和集合をとるだけなので順序も関係ない。よって 1 本の bit の
// 下位 H ビットを「行 i を塗るか」、上位 W ビットを「列 i - H を塗るか」に割り当てれば、
// 0..(1 << (H+W)) で全部の選び方をちょうど一度ずつ列挙できる。
// 数えるのは「選び方」であって「塗った結果」ではない。全行を選んだ場合と全行 + 全列を選んだ場合は
// 盤面が同じになるが、別々に数える。重複除去はしない。何も選ばない選び方も 1 通りとして数える。
//
// 盤面は concat で 1 次元にしているので、(行 r, 列 col) の添字は r * W + col になる。
// 行 i を塗るときは i * w + j (j = 0..W)、列 i を塗るときは i + w * j (j = 0..H)。
// 落とし穴: 行の添字を i * h + j と書くと、H == W のときは偶然一致するので正方形のケースでは
// 気付けない。H > W だと i * h が H*W を超えて範囲外 panic (= RE) になり、1 < H < W だと
// 範囲内のまま違うマスを塗って WA になる。Vec<Vec<char>> のまま c[r][col] で触れば、
// この添字計算そのものが要らない (下の alt 参照)。
//
// 赤は '@' で上書きしている。'#' 以外なら何でもよく、'.' でも結果は同じ。
// 出力は 1 行なので #[fastout] は本問では効いていない。

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        k: usize,
        c: [Chars; h],
    };

    let pattern = h + w;
    let table = c.concat();

    let mut cnt = 0usize;

    for bit in 0..1 << pattern {
        let mut clone_table = table.clone();
        for i in 0..pattern {
            if bit >> i & 1 == 1 {
                if i < h {
                    // 縦
                    for j in 0..w {
                        let target = i * w + j;
                        clone_table[target] = '@';
                    }
                } else {
                    let i = i - h;
                    // 横
                    for j in 0..h {
                        let target = i + w * j;
                        clone_table[target] = '@';
                    }
                }
            }
        }
        if clone_table.iter().filter(|&&c| c == '#').count() == k {
            cnt += 1;
        }
    }

    println!("{cnt}");
}

// alt: ビットを 1 本にまとめず、行用 rows と列用 cols の 2 本に分けて二重ループで回す。
//      ビット i がそのまま行 i (列 j) に対応するので「下位 H ビットが行、上位 W ビットが列」の
//      割り当ても i - h のずらしも要らない。マス (i, j) が赤くなるのは「行 i か列 j が選ばれている」
//      ときなので、塗らずに判定 1 行で済み、盤面のコピーも '@' での上書きも消える。
//      通り数は 2^H * 2^W = 2^(H+W) で AC 版と同じ
// let mut ans = 0;
// for rows in 0..1 << h {
//     for cols in 0..1 << w {
//         let mut cnt = 0;
//         for i in 0..h {
//             for j in 0..w {
//                 if rows >> i & 1 == 0 && cols >> j & 1 == 0 && c[i][j] == '#' {
//                     cnt += 1;
//                 }
//             }
//         }
//         if cnt == k {
//             ans += 1;
//         }
//     }
// }
// println!("{ans}");

// alt: 行マスクと列マスクを別々に回し、盤面をコピーせずに「行も列も選ばれていない '#'」を
//      直接数える上の版を、cartesian_product と filter().count() の二段で書いたもの。
//      可変の ans / cnt が消え、「条件を満たす選び方の個数」という問題の形がそのまま式になる
// let ans = (0..1usize << h)
//     .cartesian_product(0..1usize << w)
//     .filter(|&(rows, cols)| {
//         (0..h)
//             .cartesian_product(0..w)
//             .filter(|&(i, j)| rows >> i & 1 == 0 && cols >> j & 1 == 0 && c[i][j] == '#')
//             .count()
//             == k
//     })
//     .count();
// println!("{ans}");

// alt: 各行の黒マスを前もって u32 のビットマスクにしておくと、列を塗る操作は black[i] & !cols の
//      1 命令になり、残る黒の個数は count_ones で数えられる。内側のループが O(HW) から O(H) に
//      落ちて全体 O(2^(H+W) * H)。この制約では差が出ないが、マスを集合として扱う発想は
//      bit-dp 系にそのまま繋がる
// let black: Vec<u32> = c
//     .iter()
//     .map(|row| (0..w).filter(|&j| row[j] == '#').map(|j| 1 << j).sum())
//     .collect();
// let mut ans = 0;
// for rows in 0..1 << h {
//     for cols in 0..1u32 << w {
//         let remain: u32 = (0..h)
//             .filter(|&i| rows >> i & 1 == 0)
//             .map(|i| (black[i] & !cols).count_ones())
//             .sum();
//         if remain as usize == k {
//             ans += 1;
//         }
//     }
// }
// println!("{ans}");
