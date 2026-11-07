// algo: math
// solve: self 2026-11-09
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

// N ページを両面印刷するのに要る紙の枚数 = ceil(N / 2)。1 枚に 2 ページ載るので、
// N が偶数なら N/2 枚ぴったり、奇数なら最後の 1 ページのために 1 枚追加になる。
//
// Rust の整数除算 n / 2 は切り捨てなので、そのまま出すと奇数で 1 枚足りない
// (sample1 の N=3 で 1 を出して WA)。偶奇で分けて奇数だけ +1 するのが AC コードの形。
// (n + 1) / 2 と書いても同じで、奇数なら +1 で次の偶数に繰り上がり、偶数なら足した 1 は
// 切り捨てで消える。
//
// N <= 100 なのでオーバーフローの心配は無く usize で十分。let mut ans の mut は不要で、
// unused_mut の警告が出るだけ (動作には影響しない)。

#[fastout]
fn main() {
    input! {
        n: usize,
    };

    let mut ans = if n % 2 == 0 { n / 2 } else { n / 2 + 1 };

    println!("{ans}");
}

// alt: 構造はそのまま (ans を作って出す)。if の 2 分岐を「切り捨て + 余り」の 1 式に畳んだ
//      - if n % 2 == 0 { n / 2 } else { n / 2 + 1 } → n / 2 + n % 2 (余りが 0 か 1 なのでそのまま足せる)
//      - 再代入しないので let mut → let
// let ans = n / 2 + n % 2;
// println!("{ans}");

// alt: 切り上げ除算は標準に div_ceil がある (符号なし整数は Rust 1.73 で安定化)。
//      「ceil(N/2)」という意図がメソッド名にそのまま出るので最も読みやすく、
//      (n + 1) / 2 の +1 の意味を考える必要もない
// println!("{}", n.div_ceil(2));
