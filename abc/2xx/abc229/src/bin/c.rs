// algo: greedy, sort
// solve: self 2026-10-30
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

// N 種類のチーズ (1 g あたりのおいしさ A_i、在庫 B_i g) から合計 W g 以下を選び、
// おいしさの合計を最大化する問題。見た目はナップサックだが、チーズは 1 g 単位で
// 好きに切れるので「品物を丸ごと入れるか入れないか」の 0/1 ナップサックではない。
// 重さ W <= 3*10^8 を軸に DP すると O(NW) で論外だが、そもそも DP は要らない。
//
// 1 g ごとに見れば、どのチーズの 1 g も重さは同じで価値だけが違う。だから W g の枠は
// A_i の大きい順に埋めるのが最適。交換論法で示せる: 最適解がある A_j の 1 g を使って
// いるのに、それより A_i > A_j の i に在庫が残っているなら、j の 1 g を i の 1 g に
// 替えると重さは変わらずおいしさが A_i - A_j > 0 増え、最適性に矛盾する。
// 0/1 ナップサックで貪欲が壊れるのは「品物を分割できない」からで、本問はその前提が無い。
//
// 計算量はソートの O(N log N) (N <= 3*10^5) が支配的。ループは途中で break するので
// W が小さければ全種類を見ずに終わる。
//
// 型: 答えの上限は A_i * W = 10^9 * 3*10^8 = 3*10^17 で、i32 の 2.1*10^9 はもちろん
// u32 の 4.3*10^9 も超える。i64 / usize (64bit) なら 9.2*10^18 まで入る。
// a * num の掛け算自体も 10^9 * 1000 = 10^12 で i32 を超えるので、入力から 64bit で受ける。
//
// 落とし穴: 在庫 B_i を全部入れると W を超える最後の 1 種類は、残り容量 remain だけ
// 入れる。ここを「入らないなら飛ばす」にすると 0/1 ナップサックの発想に引きずられた
// 誤答になる。サンプル 1 (W = 5 で 4 を 2 g + 3 を 1 g + 2 を 2 g) はちょうど切る形を
// 含むので、この間違いはサンプルで気づける。

#[fastout]
fn main() {
    input! {
        n: usize,
        w: usize,
        mut ab: [(usize, usize); n],
    };

    ab.sort_unstable_by_key(|t| Reverse(t.0));

    let mut weight = 0usize;
    let mut ans = 0usize;
    for (a, b) in ab {
        if weight < w {
            let remain = w - weight;
            let num = if remain.checked_sub(b).is_some() {
                b
            } else {
                remain
            };
            ans += a * num;
            weight += num;
        } else {
            break;
        }
    }

    println!("{ans}");
}

// alt: リファクタリング版。構造 (降順ソート → for で前から詰める → 満杯で break) はそのまま
//      - `remain.checked_sub(b).is_some()` の if/else → `b.min(w - weight)` の 1 式。
//        「在庫と残り容量の小さい方」という意図がそのまま読める
//      - `if weight < w { ... } else { break }` → 先頭の早期 break にしてネストを 1 段浅く
// let mut weight = 0usize;
// let mut ans = 0usize;
// for (a, b) in ab {
//     if weight == w {
//         break;
//     }
//     let num = b.min(w - weight);
//     ans += a * num;
//     weight += num;
// }
// println!("{ans}");

// alt: 残り容量 rem を減らしながら map + sum で畳む版。Rust らしく式で答えが出る。
//      満杯になった後は take = 0 で 0 を足し続けるだけなので break 不要 (ソートが
//      O(N log N) なので末尾までの O(N) は計算量に影響しない)。weight と ans の
//      2 変数を持ち回さずに済む
// let mut rem = w;
// let ans: usize = ab
//     .into_iter()
//     .map(|(a, b)| {
//         let take = b.min(rem);
//         rem -= take;
//         a * take
//     })
//     .sum();
// println!("{ans}");
