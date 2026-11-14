// algo: math
// solve: self 2026-11-14
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

// 30 x 30 の板チョコから下の行を B 回、右の列を R 回取り除いた後の残り片数を答える問題。
// 答えは (30 - B) * (30 - R)。
//
// 行を取り除いても列数は変わらず、列を取り除いても行数は変わらない。つまり 2 つの操作は
// 互いに干渉しないので、順にシミュレーションしなくても「残った行数 × 残った列数」で出る。
// sample1 の 8 8 なら 22 * 22 = 484。
//
// 30 * 30 - B * 30 - R * 30 のように取り除いた分を引いていく形にすると、右下の B x R の
// 領域を二重に引いてしまう (sample1 で 900 - 240 - 240 = 420 になり、サンプルで気付ける)。
// 引くなら + B * R で戻す必要があるので、残る側を直接数える方が安全。
//
// 型は usize で、引き算のアンダーフローを気にする必要がある。制約 1 <= B, R <= 29 から
// 30 - B も 30 - R も 1 以上なので panic しない。答えは最大でも 29 * 29 = 841 で溢れない。

#[fastout]
fn main() {
    input! {
        b: usize,
        r: usize,
    };

    let mut ans = (30 - b) * (30 - r);

    println!("{ans}");
}

// alt: 構造はそのまま (ans を作って出す)。冗長な部分だけ削った
//      - 再代入しないので let mut → let (unused_mut の警告が消える)
// let ans = (30 - b) * (30 - r);
// println!("{ans}");
