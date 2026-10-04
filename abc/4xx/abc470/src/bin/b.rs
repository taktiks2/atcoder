// algo: counting
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

// 最終的に統一する色を x とすると、必要な操作回数は「x 以外のボール数」= N - (x の個数)。
// これを最小にするには個数が最大の色を選べばよいので、答えは N - max(度数)。O(N)。
//
// 色の値域が 1..=N (N <= 100) と小さいので、HashMap ではなく固定長配列
// [0usize; 101] をバケットにしている。スタック上に置かれヒープ確保もハッシュ計算も
// 不要で、値域が小さい整数の度数カウントではこれが最速の定石。

#[fastout]
fn main() {
    input! {
        n: usize,
        c: [usize; n],
    };

    let mut colors = [0usize; 101];

    for &ci in &c {
        colors[ci] += 1;
    }

    let ans = c.len() - colors.iter().max().unwrap();
    println!("{ans}");
}

// alt: itertools の counts() は HashMap<&T, usize> を返すので、バケット配列を
//      自分で用意せず 1 行で書ける。値域が大きい・負・文字列などでもそのまま使える
//      汎用形。今回のような小さい整数値域では配列版よりわずかに遅いが N=100 では無関係
// let ans = n - c.iter().counts().values().max().unwrap();
// println!("{ans}");

// alt: ソートして同じ値の連長を数える (要 use itertools::Itertools)。O(N log N) だが
//      追加のカウンタ領域が不要で、dedup_with_count() は (個数, 値) のタプルを返すので
//      「最頻値そのもの」も同時に欲しいときに便利
// let max_freq = c
//     .iter()
//     .sorted()
//     .dedup_with_count()
//     .map(|(cnt, _)| cnt)
//     .max()
//     .unwrap();
// println!("{}", n - max_freq);

// alt: 「統一先の色を総当たりして塗り替えコストの最小を取る」という問題構造を
//      そのまま書いた形。O(N^2) だが N <= 100 なので余裕。度数の max を取る発想が
//      出なくても、この素朴な全探索から入れば安全に AC できる
// let ans = (1..=n)
//     .map(|color| c.iter().filter(|&&ci| ci != color).count())
//     .min()
//     .unwrap();
// println!("{ans}");
