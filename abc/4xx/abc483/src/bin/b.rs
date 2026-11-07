// algo: simulation
// solve: self 2026-11-09
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

// 上から K 枚を取って「順序を逆にする」「1 枚ずつ裏返す」を Q 回行い、最終状態を出す問題。
// N, Q <= 100 なので 1 回 O(N) の愚直シミュレーションで合計 10^4 程度、工夫は要らない。
//
// 表裏を符号で持つのがポイント。カード i を表なら i、裏なら -i で持てば、裏返しは
// 符号反転、出力は値そのものになり、(番号, 向き) のペアを持たずに済む。番号が 1 始まり
// なので成り立つ書き方で、0-indexed で持つとカード 0 の表裏 (0 と -0) が区別できない。
// 負の値を持つので型は符号付き (i32)。usize では裏向きを表せない。
//
// 「逆順にする」は位置の並べ替え、「裏返す」は各要素の値の変換で互いに独立なので、
// どちらを先にやっても結果は同じ。
//
// split_at_mut で top / bottom に分けたあと concat で Vec を作り直しているので、
// 毎回 O(N) の確保が入る。制約が小さいので問題ないが、先頭 K 個のスライスに対して
// reverse と符号反転をその場で行えば確保は要らない (下の alt 参照)。

#[fastout]
fn main() {
    input! {
        n: i32,
        q: usize,
        k: [usize; q],
    };

    let mut arr: Vec<i32> = (1..=n).collect();

    for ki in k {
        let (top, bottom) = arr.split_at_mut(ki);
        top.reverse();
        let tmp: Vec<i32> = top.iter().map(|&x| x * -1).collect();
        arr = vec![tmp.as_slice(), bottom].concat();
    }

    for ele in arr {
        println!("{ele}");
    }
}

// alt: 構造はそのまま (arr を持ち、k ごとに先頭 ki 枚を逆順 + 裏返し、最後に 1 行ずつ出力)。
//      Vec を作り直さず、先頭スライスをその場で書き換える形にした
//      - split_at_mut + tmp + concat → arr[..ki].reverse() (毎回の確保が消える)
//      - x * -1 → -x、map + collect → iter_mut().for_each でその場で符号反転
// let mut arr: Vec<i32> = (1..=n).collect();
// for ki in k {
//     arr[..ki].reverse();
//     arr[..ki].iter_mut().for_each(|x| *x = -*x);
// }
// for ele in arr {
//     println!("{ele}");
// }
