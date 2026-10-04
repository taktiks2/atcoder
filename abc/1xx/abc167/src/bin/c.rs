// algo: brute-force
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

// N 冊の参考書から買う組を選び、M 個すべての理解度を X 以上にする最小金額を求める問題。
// 買う / 買わないの 2 択が N 冊分で、選び方は 2^N 通り。N <= 12 なので高々 4096 通り。
// 各通りで理解度を足し上げるのに O(NM) = 144 なので全体 O(2^N NM) ≈ 5.9 * 10^5 で余裕。
// 「安い本から買う」「理解度の合計が大きい本から買う」のような貪欲は、M 個の条件を同時に
// 満たす必要があるため成り立たない (ある本は特定のアルゴリズムにしか効かないかもしれない)。
// N だけが小さい制約はビット全探索の合図。
//
// bit の i ビット目を「i 冊目を買うか」に対応させる。bit = 0 (何も買わない) も列挙に
// 入るが、X >= 1 なので理解度 0 の状態は必ず不合格になり、答えを汚さない。
//
// 型は usize で十分。金額の最大は 12 * 10^5 = 1.2 * 10^6、理解度の最大も同じく
// 1.2 * 10^6 で、i32 の 2.1 * 10^9 にすら届かない。
//
// 落とし穴:
// - 入力の各行は C_i と A_{i,1..M} の M + 1 個なので [[usize; m + 1]; n]。[[usize]; m] と
//   書くと proconio は各行の先頭 (= C_i) を「長さ」として読み、行数も N ではなく M になる。
// - total += book[0] は if の内側に置く。外に出すと全冊の値段を毎回足してしまう
//   (この形はサンプル 1 で 180 になるので気付ける)。
// - 内側の enumerate の i は外側の i (本の番号) をシャドウしている。内側ではアルゴリズムの
//   番号として使っているだけなので動くが、名前を j にしておくと読み違えない。
// - 達成不可能なら ans は usize::MAX のまま残るので、-1 への読み替えを忘れない
//   (サンプル 2 がこのケース)。

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        x: usize,
        ca: [[usize; m+1]; n],
    };

    let mut ans = usize::MAX;

    for bit in 0..1 << n {
        let mut algos: Vec<usize> = vec![0usize; m];
        let mut total = 0usize;
        for (i, book) in ca.iter().enumerate() {
            if bit >> i & 1 == 1 {
                total += book[0];
                for (i, idea) in book[1..].iter().enumerate() {
                    algos[i] += idea;
                }
            }
        }

        let ok = algos.iter().all(|&algo| algo >= x);

        if ok {
            ans = ans.min(total);
        }
    }

    if ans == usize::MAX {
        println!("-1");
    } else {
        println!("{ans}");
    }
}

// alt: Rust らしい版。各通りを「合格なら Some(金額)、不合格なら None」に写して min を取ると、
//      戻り値が Option になり「1 通りも合格しなかった」が None として型に出る。usize::MAX の
//      番兵と最後の if が消え、-1 への読み替えは map_or 一つで済む。理解度の加算も
//      iter_mut().zip() にして添字のシャドウを無くした
// let ans = (0..1usize << n)
//     .filter_map(|bit| {
//         let chosen = ca.iter().enumerate().filter(|&(i, _)| bit >> i & 1 == 1);
//         let mut algos = vec![0; m];
//         let mut total = 0;
//         for (_, book) in chosen {
//             total += book[0];
//             for (a, &v) in algos.iter_mut().zip(&book[1..]) {
//                 *a += v;
//             }
//         }
//         algos.iter().all(|&a| a >= x).then_some(total)
//     })
//     .min();
// println!("{}", ans.map_or(-1, |v| v as i64));

// alt: itertools の powerset で買う本の組を直接列挙すると、ビット演算を一切書かずに済み最短。
//      「列ごとの和が X 以上か」を (1..=m).all で、「値段の和」を map().sum() で書くので、
//      問題文の条件がそのまま式になる。ただし部分集合ごとに Vec を確保し、列ごとに本を
//      舐め直すので定数倍は AC コードより重い (本問の規模なら問題ない)
// let ans = ca
//     .iter()
//     .powerset()
//     .filter(|books| (1..=m).all(|j| books.iter().map(|b| b[j]).sum::<usize>() >= x))
//     .map(|books| books.iter().map(|b| b[0]).sum::<usize>())
//     .min();
// println!("{}", ans.map_or(-1, |v| v as i64));
