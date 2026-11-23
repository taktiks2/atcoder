// algo: math
// solve: self 2026-11-23
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

// 威力 D 以下の攻撃をすべて防ぐ鎧が、威力 A の攻撃を防げるかを答える問題。
// 判定は A <= D の比較 1 回だけ。
//
// 落とし穴は等号の扱いだけ。「D 以下」なので A = D は防げる側で、サンプル 2 (5 5) が
// ちょうどその境界で Yes。本体は逆の条件 A > D を先に書いて No に振っているので、
// 残り (A <= D) が Yes になり等号は正しく Yes 側に入る。これを A >= D と書くと
// サンプル 2 で落ちる。
//
// A, D <= 100 なのでどの整数型でも溢れない。

#[fastout]
fn main() {
    input! {
        a: usize,
        d: usize,
    };

    let ans = if a > d { "No" } else { "Yes" };

    println!("{ans}");
}

// alt: 構造はそのまま (比較 1 回 → Yes/No)。Rust らしさ・短さの整理だけ
//      - 条件を問題文の向き (A <= D なら防げる = Yes) に揃え、a > d → "No" の二重に
//        ひっくり返した形をやめる。読むときに頭の中で否定しなくて済む
//      - ans に束縛せず、if 式をそのまま println! に渡す
// println!("{}", if a <= d { "Yes" } else { "No" });

// alt: bool を添字にして分岐を消す版。false as usize = 0, true as usize = 1 なので
//      ["No", "Yes"] の順に並べれば条件がそのまま答えを引く。Yes/No を出すだけの
//      問題で使い回せる短い書き方 (並び順を逆にすると全部ひっくり返るので注意)
// println!("{}", ["No", "Yes"][(a <= d) as usize]);
