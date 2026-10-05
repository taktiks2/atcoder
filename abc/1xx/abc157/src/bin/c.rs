// algo: brute-force
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

// N 桁の非負整数のうち「左から s_i 桁目が c_i」をすべて満たす最小のものを答える問題。
// N <= 3 なので候補は高々 900 個 (100〜999)、条件は M <= 5 個。全部作って判定しても
// 4500 回程度で終わる。条件同士の矛盾 (入力例 2) や先頭 0 (入力例 3) を場合分けで
// 潰すより、「N 桁の整数を全部試して、条件を全部満たすものがあるか」に任せた方が漏れない。
//
// 落とし穴は「N 桁」の範囲。N >= 2 なら 10^(N-1) から始めることで先頭 0 を自動で除外
// できるが、N = 1 だけは 0 も 1 桁の整数として数えるので下限を 0 にする。
// 一律に 10^(N-1) から始めると N = 1 の下限が 1 になり、「1 0」(条件なし) や
// 「1 1 / 1 0」で答え 0 を取りこぼして -1 を出す。サンプルは 3 つとも N = 3 なので
// この WA はサンプルでは見えない。
//
// 候補は昇順に試すので、最初に条件を満たしたものがそのまま最小。min を取っているのは
// 安全側だが不要。-1 を出す都合で符号付きにする必要があるが、答えは 999 以下なので
// i32 へのキャストで溢れる心配はない。出力は 1 行なので #[fastout] は本問では効いていない。

#[fastout]
fn main() {
    input! {
        n: u32,
        m: usize,
        sc: [(Usize1, usize); m],
    };

    let mut min = usize::MAX;

    let start = if n - 1 == 0 { 0 } else { 10usize.pow(n - 1) };

    for i in start..10usize.pow(n) {
        let arr: Vec<char> = format!("{}", i).chars().collect();
        let mut ok = true;
        for &(si, ci) in &sc {
            if arr[si].to_digit(10).unwrap() as usize != ci {
                ok = false;
            }
        }
        if ok {
            min = min.min(i);
        }
    }

    println!("{}", if min == usize::MAX { -1 } else { min as i32 });
}

// alt: 昇順に試すなら最初に見つかったものが最小なので、find で打ち切れる。min の番兵
//      usize::MAX と ok フラグが消え、「見つからなければ -1」が Option の None にそのまま
//      対応する。all で条件判定を書くと「全部満たす」が式に出て短い
// let lo = if n == 1 { 0 } else { 10u32.pow(n - 1) };
// let ans = (lo..10u32.pow(n)).find(|&x| {
//     let d: Vec<u32> = x.to_string().chars().map(|c| c.to_digit(10).unwrap()).collect();
//     sc.iter().all(|&(s, c)| d[s] == c as u32)
// });
// match ans {
//     Some(x) => println!("{x}"),
//     None => println!("-1"),
// }

// alt: 探索せず各桁を直接決める O(N + M) の構築版。桁ごとに Option で値を持ち、同じ桁に
//      違う値が来たら矛盾で -1。N >= 2 で先頭が 0 に指定されていたら -1。それ以外は
//      未指定の桁を最小にする (先頭は N >= 2 なら 1、他は 0)。N が大きくても通る形で、
//      全探索版がどの場合分けを暗黙に処理していたかが読める
// (n: usize, sc: [(Usize1, u32); m] で受ける)
// let mut d: Vec<Option<u32>> = vec![None; n];
// for &(s, c) in &sc {
//     if d[s].is_some_and(|x| x != c) {
//         println!("-1");
//         return;
//     }
//     d[s] = Some(c);
// }
// if n > 1 && d[0] == Some(0) {
//     println!("-1");
//     return;
// }
// if n > 1 && d[0].is_none() {
//     d[0] = Some(1);
// }
// let ans: String = d.iter().map(|x| char::from_digit(x.unwrap_or(0), 10).unwrap()).collect();
// println!("{ans}");

// alt: AC コードと同じ全探索のまま、書き方だけ整えた模範形。番兵 usize::MAX と ok フラグを
//      find / all に置き換え、-1 は Option の None で表す。桁は to_digit ではなくバイトで
//      比べる。数字の文字コードは '0'=48 から '9'=57 まで連続しているので、
//      d[s] - b'0' で「'0' から何番目か」= その桁の数値になる。to_string() の結果は
//      必ず数字だけなので、unwrap なしで安全に使える。c は u8 で受けて型を揃える
// input! {
//     n: u32,
//     m: usize,
//     sc: [(Usize1, u8); m],
// };
// let lo = if n == 1 { 0 } else { 10u32.pow(n - 1) };
// let ans = (lo..10u32.pow(n)).find(|x| {
//     let d = x.to_string().into_bytes();
//     sc.iter().all(|&(s, c)| d[s] - b'0' == c)
// });
// match ans {
//     Some(x) => println!("{x}"),
//     None => println!("-1"),
// }
