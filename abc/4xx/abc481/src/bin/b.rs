// algo: brute-force
// solve: self 2026-10-24
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

// 折れ線上で y = K となる点を、重複なく数える問題。N <= 100 なので各頂点・各線分を
// 一回ずつ見るだけで十分 (O(N))。
//
// y = K になる点は 2 種類に分かれる:
//   1. 頂点そのもの (A_i = K)
//   2. 線分 (i-1, i) の内部 (両端を除く) で K をまたぐ点
// 線分は直線なので、内部で y = K を取るのは端点の値が K を挟む (片方 < K、もう片方 > K)
// ときだけで、交点はちょうど 1 個。A_i ≠ A_{i+1} の制約から水平な線分は無く、
// 「線分全体が y = K に乗って無限個」というケースは起きない。
//
// 重複の罠は頂点。頂点 i は線分 (i-1, i) と (i, i+1) の両方の端点なので、「線分ごとに
// 端点込みで K と交わるか」を数えると A_i = K の頂点を二重に数える (サンプル 1 の
// x=2 がそれ)。このコードは頂点 i を i 番目のループでだけ数え、線分側は狭義の
// 不等号 (> / <) で端点を除外しているので、各点がちょうど一回ずつ数えられる。
// 端点の一方が K の線分は狭義不等号のどちらにも当たらず 0 個扱いになり、これも正しい。
//
// i == 0 を別扱いしているのは a[i - 1] が無いから。usize の 0 - 1 は debug ビルドで
// panic する。

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    };

    let mut ans = 0i64;

    for (i, &ai) in a.iter().enumerate() {
        if i == 0 {
            if ai == k {
                ans += 1;
            }
            continue;
        }

        if a[i] == k {
            ans += 1;
            continue;
        }

        if a[i - 1] > k && ai < k {
            ans += 1;
            continue;
        }

        if a[i - 1] < k && ai > k {
            ans += 1;
            continue;
        }
    }

    println!("{ans}");
}

// alt: 構造はそのまま (1 頂点 1 周で、頂点 → 左の線分の順に見る)。
//      - i == 0 の特別扱いと a[i] == k の判定を 1 本にまとめた (頂点の判定は i によらず共通)
//      - continue 4 本を else if にした (どれか一つに当たれば +1)
//      - 交差判定の if 2 本を || で 1 本にした
//      - a[i] を ai に揃え、ans の i64 指定を外した (答えは高々 2N - 1 = 199)
// let mut ans = 0;
// for (i, &ai) in a.iter().enumerate() {
//     if ai == k {
//         ans += 1;
//     } else if i > 0 && (a[i - 1] > k && ai < k || a[i - 1] < k && ai > k) {
//         ans += 1;
//     }
// }
// println!("{ans}");

// alt: 「頂点」と「線分内部」を別々に数えて足す宣言的な書き方。windows(2) で線分を
//      直接並べるので添字 i - 1 の境界処理が消える。交差判定は「K が端点の min と max の
//      狭義の間にある」で、<= にしない (端点を含めない) のが重複を避ける要
// let on_vertex = a.iter().filter(|&&x| x == k).count();
// let crossing = a
//     .windows(2)
//     .filter(|w| w[0].min(w[1]) < k && k < w[0].max(w[1]))
//     .count();
// println!("{}", on_vertex + crossing);
