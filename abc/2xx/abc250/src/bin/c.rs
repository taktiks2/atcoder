// algo: simulation
// solve: self 2026-11-05
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

// 位置の入れ替えを Q 回行い、最後の並びを出力する問題。シミュレーションそのものだが、
// 「値 x のボールが今どこにあるか」を素早く引けるかが全て。並び (位置 -> 値) だけを持って
// 毎回 x を線形探索すると 1 回 O(N)、全体 O(NQ) = 2×10^5 × 2×10^5 = 4×10^10 で TLE。
//
// そこで逆向きの対応 (値 -> 位置) も配列で持つ。seq_num (位置 -> 値) と seq_index (値 -> 位置) は
// 互いの逆置換なので、x の位置は seq_index[x] で O(1)。1 回の操作で変わるのは 2 箇所だけで、
// 両配列とも O(1) で更新できるから全体 O(N + Q)。
//
// 更新の順序が肝。先に seq_num.swap(t1, t2) すると seq_num[t1], seq_num[t2] には入れ替えた
// 2 つの値がそのまま入っているので、その 2 値の位置を swap すれば seq_index も正しくなる
// (swap は対称なので、どちらがどちらへ移ったかを気にしなくてよい)。片方の配列だけ更新すると
// 逆置換の関係が崩れ、次の操作から位置を誤って引く。
//
// 右端判定の seq_num[n - 1] == x は seq_index[x] == n - 1 と同値。右端なら (n - 2, n - 1)、
// それ以外は (i, i + 1) を入れ替える。N >= 2 なので n - 2 がアンダーフローすることはない。
//
// 入力は Usize1 で 0-indexed にし、出力時に +1 で戻す。値も位置も N 未満なので usize で足りる。

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        x: [Usize1; q],
    };

    // 各値がどの位置にあるか？
    let mut seq_index = Vec::from_iter(0..n);
    // 各位置にどの値があるか？
    let mut seq_num = Vec::from_iter(0..n);

    for xi in x {
        let (t1, t2) = if seq_num[n - 1] == xi {
            // 最後をswapする時
            (n - 2, n - 1)
        } else {
            // それ以外をswapする時
            let i = seq_index[xi];
            (i, i + 1)
        };
        seq_num.swap(t1, t2);
        seq_index.swap(seq_num[t1], seq_num[t2]);
    }

    let ans = seq_num.iter().map(|&x| x + 1).join(" ");

    println!("{ans}");
}

// alt: 構造はそのまま (配列 2 本、swap 2 回)。冗長な部分だけ削った版
//      - 右端判定を「値側から seq_num[n-1] を見る」→「位置 i == n-1 を見る」に変え、
//        i を先に 1 回だけ引く形に (分岐内で seq_index を引かずに済む)
//      - 相手の位置 j だけを分岐で決めるので (t1, t2) のタプルが不要に
//      - Vec::from_iter(0..n) → collect_vec()、変数名を pos / val に短縮
// let mut pos = (0..n).collect_vec(); // 値 -> 位置
// let mut val = (0..n).collect_vec(); // 位置 -> 値
// for xi in x {
//     let i = pos[xi];
//     let j = if i == n - 1 { i - 1 } else { i + 1 };
//     val.swap(i, j);
//     pos.swap(val[i], val[j]);
// }
// println!("{}", val.iter().map(|&v| v + 1).join(" "));
