// algo: brute-force, grid
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

// 出発マス N^2 個 × 方向 8 通りの数を全部作って最大を取る問題。N <= 10 なので候補は
// 高々 800 本、1 本あたり N 回進むだけで合計 8000 回程度。工夫は要らず全探索で済む。
//
// 出発マスを「最大の数字が書かれたマス」に絞っているのは、どの候補もちょうど N 桁で
// 桁数が揃っているから。桁数が同じなら先頭の桁が大きい方が必ず大きいので、先頭が
// 最大の数字でない候補は比べるまでもなく負ける。全マスから出発しても計算量は同じなので、
// 絞り込みは正しさのための処理ではなく枝刈りにすぎない。
//
// 端での回り込みを「-1 なら n-1、n なら 0」の if で書けるのは、1 歩の移動量が ±1 で、
// はみ出すのが常に 1 マスだけだから。座標を (i + k*di) のように一気に計算する書き方だと
// 大きくはみ出すので rem_euclid が要る (下の alt 参照)。その場合、Rust の % は負の数に
// 対して負を返す (-1 % 4 == -1) ので % は使えない。
//
// 型は i64 必須。N = 10 で全マス 9 なら答えは 9999999999 ≈ 10^10 で、i32 の上限
// 2.1×10^9 を超える。debug ビルドならパニックで気づけるが、ジャッジの release ビルドでは
// 黙って値が回り込み、サンプル (答えは高々 9786 / 1111111111 で i32 に収まる) は通るのに
// WA になる。実際に i32 で一度 WA を出した。座標も ci + di が負になるので符号付きで持つ。
// 出力は 1 行なので #[fastout] は本問では効いていない。

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [Chars; n],
    };

    let mut starts: Vec<Vec<(usize, usize)>> = vec![vec![]; 9];

    for (i, row) in a.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            starts[cell.to_digit(10).unwrap() as usize - 1].push((i, j));
        }
    }

    let max_index = starts.iter().rposition(|x| !x.is_empty()).unwrap();

    const DIR: [(i64, i64); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    let mut max = 0i64;
    for &(si, sj) in &starts[max_index] {
        for (di, dj) in DIR {
            let (mut ci, mut cj) = (si as i64, sj as i64);
            let mut num = a[si][sj].to_digit(10).unwrap() as i64;
            for _ in 1..n {
                let mut ti = ci + di;
                let mut tj = cj + dj;
                if ti < 0 {
                    ti = n as i64 - 1;
                } else if ti >= n as i64 {
                    ti = 0;
                }
                if tj < 0 {
                    tj = n as i64 - 1;
                } else if tj >= n as i64 {
                    tj = 0;
                }

                num = num * 10 + a[ti as usize][tj as usize].to_digit(10).unwrap() as i64;
                (ci, cj) = (ti, tj);
            }
            max = max.max(num);
        }
    }

    println!("{max}");
}

// alt: 今のコードの構造 (最大の数字のマスから出発 → 8 方向に 1 歩ずつ進む) はそのまま、
//      冗長な部分だけ削った版。
//      - 数字ごとのバケツ starts を作らず、最大の文字 top を直接求めて filter で出発マスを選ぶ
//        ('1'..='9' は char のままで数字と同じ順に比較できる)
//      - 回り込みの if 4 本を rem_euclid 2 行にまとめる
//      - 次の座標の一時変数 ti/tj を消し、ループを 0..n にして出発マスの数字もループ内で積む
//      ロジックが同じことを差分で追いやすい
// let top = *a.iter().flatten().max().unwrap();
// let mut max = 0i64;
// for (si, sj) in (0..n).cartesian_product(0..n).filter(|&(i, j)| a[i][j] == top) {
//     for (di, dj) in DIR {
//         let (mut i, mut j) = (si as i64, sj as i64);
//         let mut num = 0i64;
//         for _ in 0..n {
//             num = num * 10 + a[i as usize][j as usize].to_digit(10).unwrap() as i64;
//             i = (i + di).rem_euclid(n as i64);
//             j = (j + dj).rem_euclid(n as i64);
//         }
//         max = max.max(num);
//     }
// }
// println!("{max}");

// alt: 座標を k 歩目で (i + k*di).rem_euclid(n) と直接計算すれば、回り込みの if も
//      現在位置の更新も消える。出発マスの絞り込みもやめて N^2 × 8 本を全部作り、
//      fold で桁を積んで max を取るだけの式一つになる。計算量は同じで最短
// let n = n as i64;
// let ans = (0..n)
//     .cartesian_product(0..n)
//     .cartesian_product(DIR)
//     .map(|((i, j), (di, dj))| {
//         (0..n).fold(0u64, |acc, k| {
//             let r = (i + k * di).rem_euclid(n) as usize;
//             let c = (j + k * dj).rem_euclid(n) as usize;
//             acc * 10 + a[r][c].to_digit(10).unwrap() as u64
//         })
//     })
//     .max()
//     .unwrap();
// println!("{ans}");

// alt: 数値に直さず、N 文字の String のまま max を取る。全候補が同じ N 桁なので、
//      文字列の辞書順と数値の大小が一致する。オーバーフローがそもそも起きないので、
//      型選びのミスが構造的に消え、N が 100 でもそのまま動く
// let n = n as i64;
// let ans: String = (0..n)
//     .cartesian_product(0..n)
//     .cartesian_product(DIR)
//     .map(|((i, j), (di, dj))| {
//         (0..n)
//             .map(|k| a[(i + k * di).rem_euclid(n) as usize][(j + k * dj).rem_euclid(n) as usize])
//             .collect()
//     })
//     .max()
//     .unwrap();
// println!("{ans}");
