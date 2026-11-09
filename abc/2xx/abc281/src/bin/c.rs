// algo: prefix-sum, binary-search
// solve: self 2026-11-09
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

// N 曲を繰り返し再生して T 秒後に流れている曲と、その曲の経過秒数を求める問題。
// T <= 10^18 なので 1 秒ずつ（1 曲ずつでも）進めるのは無理。A_i >= 1 でも 1 周 N 曲を
// 10^18 回近く回すことになる。
//
// 再生は 1 周 S = ΣA_i 秒の周期なので、T 秒後の状態は T mod S 秒後の状態と同じになる。
// 周回をまとめて捨てれば、残りは「1 周目の中で remain 秒後」に落ち、1 周分だけ見ればよい。
//
// 累積和 prefix_sum[i] = 曲 1..=i の合計 とすると、曲 k が流れるのは
// [prefix_sum[k-1], prefix_sum[k]) の区間。prefix_sum は A_i >= 1 から狭義単調増加なので、
// partition_point(x < remain) が「remain 以上になる最初の添字」k を返し、
//   prefix_sum[k-1] < remain <= prefix_sum[k]
// になる。境界の扱いが本来の区間 (左閉右開) とずれているが、制約「T 秒後ちょうどに
// 曲が切り替わらない」により remain が prefix_sum のどれとも等しくならないので、
// 等号は成り立たず結果は一致する。同じ理由で remain = 0 も起きず (0 は prefix_sum[0])、
// k >= 1 が保証されて target_index - 1 がアンダーフローしない。この制約が無ければ
// T = S の倍数で panic、境界ちょうどで「前の曲の A_k 秒目」と答えてしまう。
//
// 型は u64。S は最大 10^5 × 10^9 = 10^14 で i32 の 2.1×10^9 を超え、T も 10^18 なので
// 64 bit が必須 (u64 の上限 1.8×10^19 に収まる)。
//
// 計算量は累積和の構築 O(N) が支配的。クエリは 1 回だけなので二分探索の O(log N) は
// 本質ではなく、線形に走査しても O(N) で同じ (下の alt 参照)。
// 出力は 1 行なので #[fastout] は本問では効いていない。

#[fastout]
fn main() {
    input! {
        n: usize,
        t: u64,
        a: [u64; n],
    };

    let all_time: u64 = a.iter().sum();
    let mut prefix_sum: Vec<u64> = vec![0u64; n + 1];
    let mut total = 0u64;
    for (i, ai) in a.iter().enumerate() {
        total += ai;
        prefix_sum[i + 1] = total;
    }

    let remain = t % all_time;

    let target_index = prefix_sum.partition_point(|&x| x < remain);

    println!("{} {}", target_index, remain - prefix_sum[target_index - 1]);
}

// alt: 構造はそのまま (累積和 → mod → partition_point)、冗長な部分だけ削った版
//      - total と for ループ → scan で累積和を一発で作る
//      - all_time を別に sum しない → 累積和の末尾 prefix_sum[n] が 1 周の長さそのもの
// let prefix_sum: Vec<u64> = std::iter::once(0)
//     .chain(a.iter().scan(0, |s, &x| {
//         *s += x;
//         Some(*s)
//     }))
//     .collect();
// let remain = t % prefix_sum[n];
// let target_index = prefix_sum.partition_point(|&x| x < remain);
// println!("{} {}", target_index, remain - prefix_sum[target_index - 1]);

// alt: クエリが 1 回なら累積和も二分探索も要らない。remain から曲の長さを順に引いていき、
//      引けなくなった曲が答え。計算量は同じ O(N) で配列も作らず、添字の ±1 が消える。
//      r < ai で止めるので区間が自然に左閉右開になり、境界の制約に頼らずとも正しい
// let mut r = t % a.iter().sum::<u64>();
// for (i, &ai) in a.iter().enumerate() {
//     if r < ai {
//         println!("{} {}", i + 1, r);
//         return;
//     }
//     r -= ai;
// }
