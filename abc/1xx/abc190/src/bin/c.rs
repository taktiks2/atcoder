// algo: brute-force
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

// K 人それぞれが「C_i に置くか D_i に置くか」の 2 択を選び、置き終わった最終状態に対して
// M 個の条件 (A_j, B_j が両方埋まっているか) を数え、その最大を求める問題。
// 条件の判定はボールを消費しない。皿 1 のボール 1 個が (1,2) と (1,3) の両方を満たしうる。
//
// 選び方は 2^K 通りで、K <= 16 なので高々 65536 通り。各通りで皿を埋めるのに O(K)、
// 条件を数えるのに O(M) なので全体 O(2^K (K + M)) ≈ 65536 * 116 ≈ 7.6 * 10^6 で余裕。
// 人の選択は互いに独立ではなく (同じ皿を狙うと無駄になる) 局所的な貪欲では決められないので、
// 全通りを試すのが素直かつ確実。K だけが小さい制約はビット全探索の合図。
//
// bit の i ビット目を「人 i が C_i を選ぶか」に対応させる。0..(1 << k) を回すと全組合せを
// ちょうど一度ずつ列挙できる。
//
// plates は個数で持っているが、判定は「> 0」なので bool でも同じ。同じ皿に複数人が置いても
// 1 個以上という意味しかない。
//
// 落とし穴: `bit >> i & 1 == 1` は Rust では ((bit >> i) & 1) == 1 と解釈される
// (& が == より強い)。C/C++ では & が == より弱く、同じ式が bit >> i & (1 == 1) になるので、
// 他言語から移植するときは括弧を付けておく。
// また入力は 1-indexed なので Usize1 で受けないと plates[n] で範囲外になる。

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1, Usize1); m],
        k: usize,
        cd: [(Usize1, Usize1); k],
    };

    let mut max = 0usize;

    for bit in 0..(1 << k) {
        let mut plates: Vec<usize> = vec![0; n];
        for i in 0..k {
            let (ci, di) = cd[i];
            if bit >> i & 1 == 1 {
                plates[ci] += 1;
            } else {
                plates[di] += 1;
            }
        }

        let mut cnt = 0usize;
        for (ai, bi) in &ab {
            if plates[*ai] > 0 && plates[*bi] > 0 {
                cnt += 1;
            }
        }
        max = max.max(cnt);
    }

    println!("{max}");
}

// alt: Rust らしく短い版。皿を bool で持ち、条件の数え上げを filter().count()、全通りの最大を
//      map().max() で書く。可変の max 変数と cnt 変数が消え、「全通りの最大」という問題の形が
//      そのまま式になる
// let ans = (0..1usize << k)
//     .map(|bit| {
//         let mut on = vec![false; n];
//         for (i, &(c, d)) in cd.iter().enumerate() {
//             on[if bit >> i & 1 == 1 { c } else { d }] = true;
//         }
//         ab.iter().filter(|&&(a, b)| on[a] && on[b]).count()
//     })
//     .max()
//     .unwrap();
// println!("{ans}");

// alt: ビット演算を使わず再帰で 2 択を分岐する版。人 i が置いて潜り、戻ったら取り除く
//      (バックトラック)。plates を 1 本使い回すので通りごとの Vec 確保が消える。2 択が 3 択以上に
//      増えてもビットへの対応付けを考え直さずに済むので、分岐数が一般化したときに書き換えやすい。
//      取り除く操作が要るので plates は bool ではなく個数で持つ必要がある
// fn dfs(i: usize, cd: &[(usize, usize)], ab: &[(usize, usize)], plates: &mut Vec<usize>) -> usize {
//     if i == cd.len() {
//         return ab.iter().filter(|&&(a, b)| plates[a] > 0 && plates[b] > 0).count();
//     }
//     let mut best = 0;
//     for p in [cd[i].0, cd[i].1] {
//         plates[p] += 1;
//         best = best.max(dfs(i + 1, cd, ab, plates));
//         plates[p] -= 1;
//     }
//     best
// }
// // main で: let mut plates = vec![0; n]; println!("{}", dfs(0, &cd, &ab, &mut plates));
