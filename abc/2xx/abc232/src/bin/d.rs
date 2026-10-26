// algo: dp, grid
// solve: hint 2026-10-26
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

// 左上から右か下にだけ進み、止まるまでに通れるマスの数の最大値を求める問題。
// 経路を全部たどると最大 C(198, 99) 通りになり、とても数えきれない。
//
// 移動が右か下だけなので、(i, j) に入る直前にいられるのは (i-1, j) か (i, j-1) の 2 マスだけ。
// 上の行から順に、各行を左から右へ見ていけば、この 2 マスは必ず先に決まっている。
// そこで grid[i][j] =「(i, j) まで来たときに通ったマス数」(到達できないなら 0 以下) として、
// 上と左から 1 回ずつ値を受け取る。マスは 10^4 個なので O(HW) で間に合う。
//
// 落とし穴 1: 「まだ埋めていない空きマス」「到達できないマス」「番兵」をすべて 0 で表している。
//   上と左のどちらかが 1 以上のときだけ +1 しないと、到達できないマスから値が伝わってしまう。
//   例: 1×4 の `.#..` は正解 1 だが、条件なしだと (1,3) に 1、(1,4) に 2 が入って 2 を出力する
//   (サンプルはすべて通るのに WA になる形)。
// 落とし穴 2: (1,1) は上も左も番兵の 0 なので、上の条件では埋まらない。
//   ループの前に grid[1][1] = 1 を入れておかないと、全マスが到達不能になって 0 を出力する。
//
// 実は、どの経路で来ても (i, j) までに通るマス数は i + j - 1 で決まる。
// max を取っているのは結果的に「到達できるか」の判定でしかない (2 本目の alt)。

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        c: [Chars; h],
    };

    let mut grid = vec![vec![0; w + 1]; h + 1];
    for (i, row) in c.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            grid[i + 1][j + 1] = match cell {
                '.' => 0,
                '#' => -1,
                _ => unreachable!(),
            }
        }
    }
    grid[1][1] = 1;

    let mut ans = 1;
    for i in 1..=h {
        for j in 1..=w {
            if grid[i][j] == 0 && (grid[i - 1][j] > 0 || grid[i][j - 1] > 0) {
                let up = grid[i - 1][j];
                let left = grid[i][j - 1];
                let next = up.max(left) + 1;
                grid[i][j] = next;
                ans = ans.max(next);
            }
        }
    }

    println!("{ans}");
}

// alt: 構造はそのまま (番兵付き grid に通ったマス数を書き込む)。冗長な部分を削った版
//      - match で 0 / -1 を振り分け → '#' のときだけ -1 を書く (初期値がすでに 0 なので)
//      - 条件の中と up / left で同じ添字を 2 回読む → best を 1 回だけ計算する
//      - ループ中の ans 更新 → 最後に grid.iter().flatten().max() で求める
// let mut grid = vec![vec![0; w + 1]; h + 1];
// for (i, row) in c.iter().enumerate() {
//     for (j, &cell) in row.iter().enumerate() {
//         if cell == '#' {
//             grid[i + 1][j + 1] = -1;
//         }
//     }
// }
// grid[1][1] = 1;
// for i in 1..=h {
//     for j in 1..=w {
//         let best = grid[i - 1][j].max(grid[i][j - 1]);
//         if grid[i][j] == 0 && best > 0 {
//             grid[i][j] = best + 1;
//         }
//     }
// }
// println!("{}", grid.iter().flatten().max().unwrap());

// alt: 通ったマス数は経路によらず i + j + 1 (0-indexed) で決まるので、dp の中身は bool で足りる。
//      「数」と「到達できるか」を同じ i32 に詰め込まなくて済み、0 / -1 / 番兵の区別が要らなくなる。
//      落とし穴 1・2 がそもそも起きない形
// let mut ok = vec![vec![false; w]; h];
// let mut ans = 0;
// for i in 0..h {
//     for j in 0..w {
//         ok[i][j] = c[i][j] == '.'
//             && ((i, j) == (0, 0) || (i > 0 && ok[i - 1][j]) || (j > 0 && ok[i][j - 1]));
//         if ok[i][j] {
//             ans = ans.max(i + j + 1);
//         }
//     }
// }
// println!("{ans}");
