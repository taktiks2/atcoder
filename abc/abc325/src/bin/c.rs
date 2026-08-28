// algo: grid, bfs, connected-components
use proconio::{
    input,
    marker::{Bytes, Chars},
};
use std::collections::VecDeque;

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    };

    let dir_y: [isize; 8] = [-1, -1, -1, 0, 0, 1, 1, 1];
    let dir_x: [isize; 8] = [-1, 0, 1, -1, 1, -1, 0, 1];

    let mut visited = vec![vec![false; w]; h];

    let mut ans = 0;

    for (i, row) in s.iter().enumerate() {
        for (j, &c) in row.iter().enumerate() {
            if c == '#' && !visited[i][j] {
                let mut queue = VecDeque::from([(i, j)]);
                visited[i][j] = true;
                while let Some((pos_y, pos_x)) = queue.pop_front() {
                    for dir in 0..8 {
                        let (dy, dx) = (dir_y[dir], dir_x[dir]);
                        let (ty, tx) = (pos_y as isize + dy, pos_x as isize + dx);
                        if ty < 0 || ty >= h as isize || tx < 0 || tx >= w as isize {
                            continue;
                        }
                        let (ty, tx) = (ty as usize, tx as usize);
                        if s[ty][tx] == '#' && !visited[ty][tx] {
                            queue.push_back((ty, tx));
                            visited[ty][tx] = true;
                        }
                    }
                }
                ans += 1;
            } else {
                visited[i][j] = true
            }
        }
    }

    println!("{ans}");
}

// alt: main のリファクタ版 — Bytes 入力・visited の 1 次元化・checked_add_signed で isize キャスト排除・不要な else 分岐の削除
#[allow(dead_code)]
fn main_refactored() {
    input! {
        h: usize,
        w: usize,
        s: [Bytes; h], // Vec<u8> になる。b'#' 比較は char より軽く、グリッドの定石
    };

    // 方向 2 本持ちをタプル 1 本に。y/x の添字ズレが起きない
    const DIRS: [(isize, isize); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    // Vec<Vec<bool>> はアロケーションが h+1 回・キャッシュも不利。1 次元 + i*w+j が定石
    let mut visited = vec![false; h * w];
    let mut ans = 0;

    for i in 0..h {
        for j in 0..w {
            if s[i][j] != b'#' || visited[i * w + j] {
                continue; // 早期 continue で else { visited[..] = true } が不要になる（'.' の visited は誰も読まない）
            }
            ans += 1;
            visited[i * w + j] = true;
            let mut queue = VecDeque::from([(i, j)]);
            while let Some((y, x)) = queue.pop_front() {
                for (dy, dx) in DIRS {
                    // checked_add_signed: 負方向へのはみ出しは None になるので isize キャストと ty < 0 判定が消える
                    let (Some(ty), Some(tx)) = (y.checked_add_signed(dy), x.checked_add_signed(dx))
                    else {
                        continue;
                    };
                    if ty < h && tx < w && s[ty][tx] == b'#' && !visited[ty * w + tx] {
                        visited[ty * w + tx] = true;
                        queue.push_back((ty, tx));
                    }
                }
            }
        }
    }

    println!("{ans}");
}

// alt: ac-library の Dsu なら visited・キュー管理が丸ごと消える。merge は対称なので右・下側 4 近傍だけ張れば 8 近傍連結になる
// use ac_library::Dsu;
// use proconio::{input, marker::Chars};
//
// fn main() {
//     input! {
//         h: usize,
//         w: usize,
//         s: [Chars; h],
//     };
//
//     let mut dsu = Dsu::new(h * w);
//     for i in 0..h {
//         for j in 0..w {
//             if s[i][j] != '#' {
//                 continue;
//             }
//             for (dy, dx) in [(0, 1), (1, -1), (1, 0), (1, 1)] {
//                 let (ty, tx) = (i as isize + dy, j as isize + dx);
//                 if (0..h as isize).contains(&ty)
//                     && (0..w as isize).contains(&tx)
//                     && s[ty as usize][tx as usize] == '#'
//                 {
//                     dsu.merge(i * w + j, ty as usize * w + tx as usize);
//                 }
//             }
//         }
//     }
//     let ans = (0..h * w)
//         .filter(|&v| s[v / w][v % w] == '#' && dsu.leader(v) == v)
//         .count();
//     println!("{ans}");
// }

// alt: 方向配列 2 本は (-1..=1) の直積で生成できる・y と x の添字ズレが構造的に起きない
// let dirs: Vec<(isize, isize)> = (-1..=1)
//     .flat_map(|dy| (-1..=1).map(move |dx| (dy, dx)))
//     .filter(|&d| d != (0, 0))
//     .collect();
// // ループ側は for &(dy, dx) in &dirs { ... } になり dir_y[dir]/dir_x[dir] の引き当てが消える
