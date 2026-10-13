// algo: deque, simulation
// solve: hint 2026-10-13
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

// 筒の右からボールを追加し、左から取り出して総和を求めるクエリ処理問題。
//
// ボールを 1 個ずつキューに追加・取り出しすると、c <= 10^9 より 1 クエリで
// 最大 10^9 個の出し入れが発生する。最悪で Q * c ≈ 2×10^5 × 10^9 = 2×10^14 回の
// 操作となり TLE かつ MLE。
//
// 同じ数 x が書かれたボールがまとめて追加される性質に着目し、(値, 個数) のペアとして
// 両端キュー (VecDeque) でまとめて保持する。
// 追加 (タイプ 1) は 1 まとまりを末尾に push_back するだけで O(1)。
// 取り出し (タイプ 2) は先頭からまとまりを参照し、要求数 c に応じて個数を減らすか、
// 使い切ったら pop_front する。各まとまりは筒に高々 1 回追加され、高々 1 回
// 完全に消費されて消えるため、取り出しにかかる全体の操作回数はならし O(Q)。
// 全体で O(Q) で高速に処理できる。
//
// 型は i64 が必須。取り出したボールの合計値は最大で Q * x * c 規模になり、
// サンプル 2 のように 1 回のクエリでも 10^9 × 10^9 = 10^18 に達するため、
// i32 (上限 ≈ 2.1×10^9) ではオーバーフローする。
//
// 先頭のまとまりの個数 cnt が必要数 c より多いときは、キューから取り出さずに
// 残り個数を ball[0] = (value, cnt - c) と更新して抜けること。
// 出力行数は最大で Q = 2×10^5 行に及ぶため、#[fastout] によるバッファリングが効く。

#[fastout]
fn main() {
    input! {
        q: usize,
    };

    // (値, 個数)
    let mut ball: VecDeque<(i64, i64)> = VecDeque::new();

    for _ in 0..q {
        input! {
            q_type: u8,
        }

        match q_type {
            1 => {
                input! {
                    x: i64,
                    c: i64,
                }
                ball.push_back((x, c));
            }
            2 => {
                input! {
                    mut c: i64,
                }
                let mut total = 0i64;

                while let Some(&(value, cnt)) = ball.front() {
                    if cnt > c {
                        total += c * value;
                        ball[0] = (value, cnt - c);
                        break;
                    } else {
                        total += cnt * value;
                        c -= cnt;
                        ball.pop_front();
                    }
                }

                println!("{total}");
            }
            _ => unreachable!(),
        }
    }
}

// alt: 構造はそのままリファクタリング。取り出す個数 take = c.min(cnt) で共通化し、
//      重複コードを整理した形。
//      - total への加算と c の減算を take 経由で 1 回に集約
//      - ball[0] へのタプル再代入を ball[0].1 -= take に簡約
//      - 全量消費 (cnt == take) と一部消費 (else) の境界を明確化
// let mut total = 0i64;
// while let Some(&(value, cnt)) = ball.front() {
//     let take = c.min(cnt);
//     total += take * value;
//     c -= take;
//     if cnt == take {
//         ball.pop_front();
//     } else {
//         ball[0].1 -= take;
//         break;
//     }
// }
// println!("{total}");

// alt: pop_front と push_front だけで書く。参照や添字アクセスを使わず、一度取り出して
//      余りがあれば先頭に戻すキュー操作の標準的スタイル。ループ条件も while c > 0 で
//      直感的になり、front() の Option 外しや [0] の直接書き換えが不要になる
// let mut total = 0i64;
// while c > 0 {
//     let (value, cnt) = ball.pop_front().unwrap();
//     let take = c.min(cnt);
//     total += take * value;
//     c -= take;
//     if cnt > take {
//         ball.push_front((value, cnt - take));
//     }
// }
// println!("{total}");

// alt: enum と Readable で入力を一括受領。入力パースとシミュレーションの責務を分離し、
//      match 式でクエリを宣言的に処理する。クエリごとに引数の個数が異なる形式に対する
//      Rust らしい型安全なアプローチ
// enum Query {
//     Push(i64, i64),
//     Pop(i64),
// }
// impl proconio::source::Readable for Query {
//     type Output = Query;
//     fn read<R: std::io::BufRead, S: proconio::source::Source<R>>(source: &mut S) -> Query {
//         let t = u8::read(source);
//         if t == 1 {
//             Query::Push(i64::read(source), i64::read(source))
//         } else {
//             Query::Pop(i64::read(source))
//         }
//     }
// }
// input! {
//     q: usize,
//     queries: [Query; q],
// }
// for query in queries {
//     match query {
//         Query::Push(x, c) => ball.push_back((x, c)),
//         Query::Pop(mut c) => {
//             let mut total = 0i64;
//             while c > 0 {
//                 let (value, cnt) = ball.pop_front().unwrap();
//                 let take = c.min(cnt);
//                 total += take * value;
//                 c -= take;
//                 if cnt > take {
//                     ball.push_front((value, cnt - take));
//                 }
//             }
//             println!("{total}");
//         }
//     }
// }
