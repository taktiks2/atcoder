// algo: grid, brute-force
// solve: self 2026-11-21
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

// N 個の蛇口と M 個の排水口のうち、ちょうど 1 つだけ詰まっている。S[i][j] は
// 「蛇口 i から注いで排水口 j から出るか」。この表から詰まっている 1 つを当てる問題。
//
// 詰まりが 1 つだけなので、表の形は 2 通りしかない。
//   - 蛇口 i が詰まっている   → 行 i が全部 x、他は全部 o
//   - 排水口 j が詰まっている → 列 j が全部 x、他は全部 o
// よって「全部 x の行があるか」を先に調べ、あればその行番号が蛇口。無ければ排水口で、
// x を 1 つ見つければその列番号が答え (x は 1 列にしか無いので、どの x でも同じ列)。
//
// 行を先に調べる順序が正しい根拠は M >= 2。排水口が詰まっている場合、各行の x は
// 1 個だけで残り M - 1 >= 1 個は o なので、「全部 x の行」は蛇口の場合にしか現れない。
// (M = 1 だと全部 x の 1 列と区別できないが、制約と「答えが特定できる」で除外される)
// 逆に x を先に探して列番号を出す順にすると、蛇口の場合でも x が見つかってしまい
// サンプル 1 で Drain 1 と答えて落ちる。
//
// 計算量は O(NM) で、N, M <= 100 なら 10^4。出力は 1-indexed なので + 1 を忘れない。

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        s: [Chars; n],
    };

    for (i, si) in s.iter().enumerate() {
        if si.iter().all(|&x| x == 'x') {
            println!("Faucet {}", i + 1);
            return;
        }
    }

    for i in (0..n) {
        for j in (0..m) {
            if s[i][j] == 'x' {
                println!("Drain {}", j + 1);
                return;
            }
        }
    }
}

// alt: 構造はそのまま (全部 x の行を探す → 無ければ x の列を探す)。冗長の整理だけ
//      - for i in (0..n) の括弧は不要 (unused_parens の警告が出る)
//      - 排水口が詰まっていれば列 j は全行で x なので、1 行目だけ見れば必ず見つかる。
//        2 重ループの外側を消して s[0] だけ走査する (添字 s[i][j] も enumerate に置換)
// for (i, si) in s.iter().enumerate() {
//     if si.iter().all(|&x| x == 'x') {
//         println!("Faucet {}", i + 1);
//         return;
//     }
// }
// for (j, &c) in s[0].iter().enumerate() {
//     if c == 'x' {
//         println!("Drain {}", j + 1);
//         return;
//     }
// }

// alt: 「条件を満たす最初の位置」は position で直接取れる。ループと早期 return が消え、
//      if let の 2 分岐が「蛇口か排水口か」という場合分けそのものになる。unwrap は
//      「全部 x の行が無いなら 1 行目に必ず x がある」という上の考察が根拠
// if let Some(i) = s.iter().position(|row| row.iter().all(|&c| c == 'x')) {
//     println!("Faucet {}", i + 1);
// } else {
//     let j = s[0].iter().position(|&c| c == 'x').unwrap();
//     println!("Drain {}", j + 1);
// }
