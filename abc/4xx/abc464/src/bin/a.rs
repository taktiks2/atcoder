// algo: counting, string
// solve: self 2026-11-19
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

// E と W のうち多い方を答える問題。W の個数 cnt を数えれば、E の個数は |S| - cnt で
// 決まるので、数えるのは片方だけでよい。
//
// |S| は奇数と制約されているので、E と W が同数になることは無い。だから
// 「W が多い」の否定はそのまま「E が多い」で、else に引き分けの分岐は要らない。
// > を >= にしても結果が変わらないのも同じ理由。
//
// s.len() - cnt は usize の引き算だが、cnt <= s.len() が必ず成り立つので
// アンダーフローしない。|S| <= 99 なので 1 回舐める O(|S|) で十分。

#[fastout]
fn main() {
    input! {
        s: Chars,
    };

    let cnt = s.iter().filter(|&&x| x == 'W').count();

    let ans = if cnt > s.len() - cnt { "West" } else { "East" };

    println!("{ans}");
}

// alt: 構造はそのまま (W を数えて多数派を判定)。比較の書き方だけ整理
//      - cnt > s.len() - cnt を移項して cnt * 2 > s.len() に。引き算が消えるので
//        usize のアンダーフローを考える必要自体が無くなる
//      - ans に束縛せず、if 式をそのまま println! に渡す
// let cnt = s.iter().filter(|&&x| x == 'W').count();
// println!("{}", if cnt * 2 > s.len() { "West" } else { "East" });

// alt: Chars (Vec<char>) にせず String のまま str::matches で数える版。パターンに
//      一致する部分を返すイテレータなので、filter + クロージャが 1 メソッドになり短い。
//      文字単位の添字アクセスが要らない問題なら String で受ける方が Rust らしい
// input! { s: String }
// let cnt = s.matches('W').count();
// println!("{}", if cnt * 2 > s.len() { "West" } else { "East" });

// alt: 個数を 2 つ比べる代わりに W を +1、E を -1 として差だけを持つ版。合計が正なら
//      West。|S| が奇数なので合計は 0 にならない。「多数決は差の符号」という見方は
//      文字種が 2 つの問題でそのまま使い回せる
// input! { s: String }
// let diff: i32 = s.bytes().map(|c| if c == b'W' { 1 } else { -1 }).sum();
// println!("{}", if diff > 0 { "West" } else { "East" });
