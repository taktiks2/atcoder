// algo: counting, parity
// solve: self 2026-10-31
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

// S 中の 'o' の個数の偶奇を答える問題。|S| <= 10 なので数えるだけ。
//
// 'o' が 0 個のケース (サンプル 3 の ".") は even。0 は偶数なので cnt % 2 == 0 の
// 判定でそのまま拾える。0 個を特別扱いして odd や別出力にすると落ちる。
// cnt は usize で、最大 10 なので型の心配は無い。

#[fastout]
fn main() {
    input! {
        s: Chars,
    };

    let cnt = s.iter().filter(|&&c| c == 'o').count();

    println!("{}", if cnt % 2 == 0 { "even" } else { "odd" });
}

// alt: リファクタリング版なし — filter + count + 偶奇判定の 3 手で、これ以上削れる冗長さが無い

// alt: 短さ。String で受けて str::matches で数える。Chars (Vec<char>) を作らずに済み、
//      「部分文字列の出現回数」を数える定番の書き方としても使い回せる
// input! {
//     s: String,
// };
// let cnt = s.matches('o').count();
// println!("{}", if cnt % 2 == 0 { "even" } else { "odd" });

// alt: 欲しいのは個数ではなく偶奇だけ、という問題の形をそのまま書く版。
//      'o' を見るたびに bool を XOR で反転させれば、最後に残るのが「奇数個か」になる。
//      個数が巨大でも溢れない (偶奇だけを持ち回す) 考え方は parity 系の問題で効く
// let odd = s.iter().fold(false, |p, &c| p ^ (c == 'o'));
// println!("{}", if odd { "odd" } else { "even" });
