// algo: brute-force
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

// 順列 P, Q が辞書順で何番目かを求め、その差 |a - b| を出す問題。N <= 8 なので
// 全順列は 8! = 40320 個しかなく、全部並べて番号を振るだけで間に合う。1 本あたり
// N 要素の比較をするので全体で O(N! * N) ≈ 3.2×10^5。
//
// 正しさは「itertools の permutations が辞書順で出てくるか」にかかっている。
// permutations は入力イテレータの位置で辞書順に出す仕様なので、入力の 1..=n が
// 昇順であれば出力も値の辞書順になる。入力が昇順でない場合（例: p をそのまま
// permutations に渡す）は順番がずれて WA になる。
//
// 番号は 0 始まりの enumerate を使っているが、差を取るので 0 始まりか 1 始まりかは
// 答えに影響しない。符号付きで引くために i32 にしている。最大値は 8! - 1 = 40319
// なので i32 で余裕があり、usize のまま a - b とするとアンダーフローで panic する。
//
// P == Q のときは a == b で答えは 0 になる（サンプル 3）。見つかった時点で break
// していないので、最後まで回しても結果は変わらない。

#[fastout]
fn main() {
    input! {
        n: usize,
        p: [usize; n],
        q: [usize; n],
    };

    let mut a = 0i32;
    let mut b = 0i32;
    for (i, v) in (1..=n).permutations(n).enumerate() {
        if v == p {
            a = i as i32;
        }
        if v == q {
            b = i as i32;
        }
    }

    let ans = (a - b).abs();
    println!("{ans}");
}

// alt: collect して position で探す形。可変変数と if が消え、「何番目か」という
//      問いがそのままコードの形になる。全順列を Vec に持つがメモリは 40320×8 要素程度
// let perms = (1..=n).permutations(n).collect_vec();
// let rank = |t: &Vec<usize>| perms.iter().position(|v| v == t).unwrap() as i32;
// let ans = (rank(&p) - rank(&q)).abs();

// alt: 列挙せずに順位を直接計算する（Lehmer code / 階乗進数）。先頭 i 番目に p[i] より
//      小さい未使用の値を置いた順列はすべて P より前にあり、それぞれ (n-1-i)! 個ずつある。
//      「未使用で p[i] より小さい値」は p[i+1..] のうち p[i] より小さいものの個数と
//      一致する。O(N^2) なので N が大きくても使え、N! 個の列挙が不要になる
// let fact: Vec<i32> = (0..n)
//     .scan(1, |f, i| {
//         if i > 0 {
//             *f *= i as i32;
//         }
//         Some(*f)
//     })
//     .collect();
// let rank = |t: &[usize]| -> i32 {
//     (0..n)
//         .map(|i| t[i + 1..].iter().filter(|&&x| x < t[i]).count() as i32 * fact[n - 1 - i])
//         .sum()
// };
// let ans = (rank(&p) - rank(&q)).abs();
