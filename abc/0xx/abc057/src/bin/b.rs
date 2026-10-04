// algo: brute-force
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

// 各学生について、マンハッタン距離が最小のチェックポイントの番号を答える問題。
// N, M <= 50 なので全ペアを調べても N*M = 2500 回。工夫の余地は無く、全探索がそのまま正解。
//
// 型: 座標は |x| <= 10^8 なので距離は最大 |a-c| + |b-d| <= 2*10^8 + 2*10^8 = 4*10^8。
// i32 の上限 2.1*10^9 に収まるので、実は i32 でも溢れない。i64 は安全側に倒しただけ。
// 一方で負の座標があるので、usize で読むと input! の時点で落ちる。符号付きは必須。
//
// 落とし穴はタイブレーク。「最も近いものが複数あれば番号が最小」なので、更新条件は
// min > total (厳密に小さいときだけ更新) にする。>= にすると同距離の後ろの番号で上書きされ、
// 最大番号を返してしまう。サンプル 2 の学生 1 (10,10) はチェックポイント 3, 4 が同じ座標
// (3,5) で同距離 12 になるため、>= だと 4 を出して落ちる。ここはサンプルで検出できる。
//
// min の初期値 i64::MAX は「最初のチェックポイントで必ず更新される」ための番兵。
// M >= 1 なので p = 0 のまま残ることは無い。

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(i64, i64); n],
        cd: [(i64, i64); m],
    };

    for (ai, bi) in ab {
        let mut min = i64::MAX;
        let mut p = 0;
        for (i, (ci, di)) in cd.iter().enumerate() {
            let total = (ai - ci).abs() + (bi - di).abs();

            if min > total {
                min = total;
                p = i;
            }
        }
        println!("{}", p + 1);
    }
}

// alt: min_by_key は等しい最小が複数あると最初の要素を返す (ドキュメントで保証)。
//      そのため「番号最小を選ぶ」タイブレークが API の仕様にそのまま乗り、min / p の
//      2 変数管理と番兵が消える。Rust らしく短い
// for (a, b) in ab {
//     let j = (0..m)
//         .min_by_key(|&j| (a - cd[j].0).abs() + (b - cd[j].1).abs())
//         .unwrap();
//     println!("{}", j + 1);
// }

// alt: (距離, 番号) のタプルで min を取る版。タプルの辞書順比較により「距離が同じなら
//      番号が小さい方」がキーに明示されるので、min_by_key の仕様を覚えていなくても
//      正しさが読める。座標を i32 で読んでも 4*10^8 で溢れないので、型も i32 で足りる
// input! { n: usize, m: usize, ab: [(i32, i32); n], cd: [(i32, i32); m] };
// for (a, b) in ab {
//     let (_, j) = cd
//         .iter()
//         .enumerate()
//         .map(|(j, &(c, d))| ((a - c).abs() + (b - d).abs(), j))
//         .min()
//         .unwrap();
//     println!("{}", j + 1);
// }
