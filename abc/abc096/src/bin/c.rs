// algo: grid
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

// 「隣接 2 マスを両方黒く塗る」操作だけで目標の白黒を作れるかを判定する問題。
// 操作の順番や回数を探索しようとすると手に負えないので、作れる盤面の特徴づけに回る。
//
// 必要条件: 上下左右がすべて白(または盤外)の黒マスがあると作れない。その黒を塗る操作では
// 相方の隣接マスも必ず黒くなるが、相方は白のまま残す必要があり、一度黒くしたマスは白に戻せない。
// 盤面の大きさとは関係なく成り立つ(1x1 の "#" も、50x50 の中の孤立した 1 マスも No)。
// 市松模様(サンプル 2)は、黒がすべて孤立している特殊な場合にすぎない。
//
// 十分条件: どの黒マスにも黒の隣人がいれば、黒マスごとに「自分と隣の黒」を 1 回ずつ塗れば
// 完成する。塗るのは黒にすべきマスだけなので、白にすべきマスは汚れない。同じマスを何度も
// 塗ることになるが、問題文に「すでに黒く塗られているマスを選ぶこともでき」とあるので問題ない。
// この但し書きが無ければ、黒マスを重複なく隣接ペアに分ける完全マッチングの問題になり、
// 別物になる。
//
// よって「孤立した黒が 1 つも無い」を各マスの 4 近傍で確かめるだけ。H, W <= 50 なので
// 高々 2500 マス x 4 方向で、計算量はまったく問題にならない。BFS や連結成分は要らない。
//
// 落とし穴: 端のマスでは近傍が盤外に出る。usize のまま i - 1 を計算すると i = 0 でパニック
// するので、isize に直してから範囲をチェックしている。

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    };

    // 十字探索用
    let dir_x: [isize; 4] = [-1, 0, 0, 1];
    let dir_y: [isize; 4] = [0, -1, 1, 0];

    for (i, row) in s.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell == '#' {
                let mut has_black = false;
                for ti in 0..4 {
                    let ty = i as isize + dir_y[ti];
                    let tx = j as isize + dir_x[ti];
                    if ty >= 0 && ty < h as isize && tx >= 0 && tx < w as isize {
                        if s[ty as usize][tx as usize] == '#' {
                            has_black = true;
                        }
                    }
                }
                if !has_black {
                    println!("No");
                    return;
                }
            }
        }
    }
    println!("Yes");
}

// alt: AC コードの形(二重ループ + 早期 return)を保ったまま整えた版。dir_x / dir_y の 2 配列を
//      (di, dj) のタプル 1 本の const にまとめ、x と y の取り違えを防ぐ。has_black は mut フラグを
//      やめて any で書く(黒が見つかった時点で打ち切られる)。checked_add_signed が 0 未満を
//      None で弾くので、残る範囲チェックは上限側の ni < h だけ。白マスは continue で先に
//      抜けてネストを 1 段減らす
// const DIRS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
// for (i, row) in s.iter().enumerate() {
//     for (j, &cell) in row.iter().enumerate() {
//         if cell != '#' {
//             continue;
//         }
//         let has_black = DIRS.iter().any(|&(di, dj)| {
//             match (i.checked_add_signed(di), j.checked_add_signed(dj)) {
//                 (Some(ni), Some(nj)) => ni < h && nj < w && s[ni][nj] == '#',
//                 _ => false,
//             }
//         });
//         if !has_black {
//             println!("No");
//             return;
//         }
//     }
// }
// println!("Yes");

// alt: all / any で「全黒マスについて、黒の隣人がいずれかの方向にいる」という条件をそのまま
//      式に写した版。フラグ変数と早期 return が消える。!0 (= usize::MAX) を足して
//      wrapping_add すると -1 の代わりになり、盤外は ni < h 一つで弾けるので isize への
//      往復キャストも不要
// let ok = (0..h).all(|i| {
//     (0..w).all(|j| {
//         s[i][j] == '.'
//             || [(0, 1), (1, 0), (0, !0), (!0, 0)].iter().any(|&(di, dj)| {
//                 let (ni, nj) = (i.wrapping_add(di), j.wrapping_add(dj));
//                 ni < h && nj < w && s[ni][nj] == '#'
//             })
//     })
// });
// println!("{}", if ok { "Yes" } else { "No" });

// alt: 盤面の周りに '.' の番兵を 1 周巻く版。近傍が必ず配列内に収まるので範囲チェックが丸ごと消え、
//      i - 1 も 1..=h の範囲では安全になる。グリッド問題で境界の場合分けを減らす定番の手
// let mut g = vec![vec!['.'; w + 2]; h + 2];
// for i in 0..h {
//     for j in 0..w {
//         g[i + 1][j + 1] = s[i][j];
//     }
// }
// for i in 1..=h {
//     for j in 1..=w {
//         if g[i][j] == '#'
//             && g[i - 1][j] != '#'
//             && g[i + 1][j] != '#'
//             && g[i][j - 1] != '#'
//             && g[i][j + 1] != '#'
//         {
//             println!("No");
//             return;
//         }
//     }
// }
// println!("Yes");
