// algo: simulation
// solve: self 2026-10-12
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

// 奇数番目 (1-indexed) の項の総和。N <= 100 なので 1 周するだけ。
//
// 落とし穴は添字のずれ。enumerate の i は 0 始まりなので、問題の「i 番目」は i + 1 になる。
// (i + 1) % 2 == 1 は i % 2 == 0 と同じで、0-indexed では「偶数添字を足す」になる。
// i % 2 == 1 と書くと偶数番目の和になる。サンプル 1 は 12 のはずが 2 と出るので、
// ここはサンプルで気付ける。
//
// 値は高々 100 × 50 = 5000 なので型は何でも良い。
// 出力は 1 行なので #[fastout] は本問では効いていない。

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };

    let mut ans = 0usize;
    for (i, ai) in a.iter().enumerate() {
        if (i + 1) % 2 == 1 {
            ans += ai;
        }
    }

    println!("{ans}");
}

// alt: 構造はそのまま。削ったのは:
//      - (i + 1) % 2 == 1 → i % 2 == 0 (0-indexed のまま判定する)
//      - 0usize の型注釈 (ans += ai から usize に推論される)
// let mut ans = 0;
// for (i, ai) in a.iter().enumerate() {
//     if i % 2 == 0 {
//         ans += ai;
//     }
// }

// alt: step_by(2) で「先頭から 1 つおき」を直接取る。添字の偶奇判定そのものが消えるので、
//      0/1-indexed の取り違えが起きる余地がない。最短
// let ans: usize = a.iter().step_by(2).sum();
