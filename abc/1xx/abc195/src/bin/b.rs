// algo: brute-force
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

// みかん (各 A〜B グラム、重さは実数) を何個か選んで合計 W キログラムにするとき、個数の
// 最小と最大を答える問題。単位が違うので最初に W を 1000 倍してグラムに揃える。
//
// 個数 x を固定すると、合計は「全部 A」の xA から「全部 B」の xB まで動き、重さは実数
// なのでその間はどの値も作れる (全員を同じ割合で連続的に重くしていけばよい)。よって
// x 個で W にできる ⇔ xA <= W <= xB。これを x = 1, 2, ... と試して、最初に通った x が
// 最小、最後に通った x が最大。
//
// x の上限は xA <= W から W/A。W <= 10^6, A >= 1 なので最悪 10^6 回で十分間に合う。
// x*b も高々 (10^6+1)*1000 ≈ 10^9 で usize (64bit) には余裕がある。
// n の .max(total_gram / b + 1) 側は B >= A なので常に小さく、効いていない。
//
// 落とし穴は「1 個だけで調整する」発想。x-1 個を A (または B) に固定し残り 1 個で差を
// 埋められるか、という判定にすると、入力例 2 (120 150 2) で 16 個 = 全部 125g のような
// 「全員を少しずつ動かす」解を取りこぼして UNSATISFIABLE になる。x = 1 も判定から漏れる。
// 実数なので「各個数で作れる合計の区間」だけを見れば足りる。
//
// 通る x の集合は ceil(W/B) <= x <= floor(W/A) という連続区間なので、min だけ見つかって
// max が見つからないことはない。match の (true, false) / (false, true) には入らない。
// 出力は 1 行なので #[fastout] は本問では効いていない。

#[fastout]
fn main() {
    input! {
        a: usize,
        b: usize,
        w: usize,
    };

    let mut min = usize::MAX;
    let mut max = 0usize;

    let total_gram = w * 1000;

    let n = (total_gram / a + 1).max(total_gram / b + 1);

    for x in 1..=n {
        if x * a <= total_gram && x * b >= total_gram {
            if min == usize::MAX {
                min = x;
            }
            max = x;
        }
    }

    match (min == usize::MAX, max == 0) {
        (true, true) => {
            println!("UNSATISFIABLE");
        }
        (true, false) => {
            println!("{} {}", max, max);
        }
        (false, true) => {
            println!("{} {}", min, min);
        }
        (false, false) => {
            println!("{} {}", min, max);
        }
    }
}

// alt: find と rev().find で最小・最大を直接取る。番兵 usize::MAX / 0 と 4 通りの match が
//      消え、「見つからなければ UNSATISFIABLE」が Option の None にそのまま対応する
// let w = w * 1000;
// let ok = |&x: &usize| a * x <= w && w <= b * x;
// match ((1..=w / a).find(ok), (1..=w / a).rev().find(ok)) {
//     (Some(lo), Some(hi)) => println!("{lo} {hi}"),
//     _ => println!("UNSATISFIABLE"),
// }

// alt: ループ不要の O(1) 版。xA <= W <= xB を x について解くと W/B <= x <= W/A なので、
//      最小は ceil(W/B)、最大は floor(W/A)。この区間が空 (min > max) なら UNSATISFIABLE。
//      切り上げは div_ceil (Rust 1.73+) で書ける
// let w = w * 1000;
// let (lo, hi) = (w.div_ceil(b), w / a);
// if lo <= hi {
//     println!("{lo} {hi}");
// } else {
//     println!("UNSATISFIABLE");
// }

// alt: AC コードと同じ for ループの全探索のまま、書き方だけ整えた模範形。
//      - w をグラムで上書き (シャドーイング) し、total_gram と w の二重管理をやめる
//      - 上限は w / a だけで足りる (B >= A なので w / b 側は常に小さい)
//      - 条件を a * x <= w && w <= b * x と小さい順に並べ、「w が区間に入るか」と読めるようにする
//      - 番兵 usize::MAX / 0 を Option<(lo, hi)> にまとめる。「min だけある」状態が型の上で
//        作れなくなり、到達しない match の 2 分岐が消える
// input! {
//     a: usize,
//     b: usize,
//     w: usize,
// };
// let w = w * 1000; // kg -> g
// let mut ans: Option<(usize, usize)> = None;
// for x in 1..=w / a {
//     if a * x <= w && w <= b * x {
//         let lo = ans.map_or(x, |(lo, _)| lo);
//         ans = Some((lo, x));
//     }
// }
// match ans {
//     Some((lo, hi)) => println!("{lo} {hi}"),
//     None => println!("UNSATISFIABLE"),
// }
