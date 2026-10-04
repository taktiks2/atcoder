// algo: enumeration, binary-search
#![allow(unused_imports)]
use indexing::algorithms::binary_search;
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

// 326-like number は「百の位 × 十の位 == 一の位」を満たす 3 桁数。100..=919 を全部試して
// 集めると 32 個しかなく、しかも昇順に並ぶ。あとはその中から N 以上で最小のものを取れば
// よく、ソート済み配列なので二分探索が効く。
//
// 注意すべきは「N, N+1, N+2, ... を直接二分探索することはできない」点。二分探索の前提は
// 述語の単調性 (false が続いた後 true が続く) だが、「x が 326-like か」は 100:yes 101:no
// 111:yes ... とバラバラで単調でない。軸を「x の値」から「昇順に並べた候補列の添字」へ
// 移して初めて、述語「like_numbers[i] >= N」が i について単調になり二分探索が成立する。
//
// 探索部は半開区間 [start, end) 形式で、ループ中の不変条件は
//   添字 <  start … like_numbers[i] <  N と確定 (捨てた)
//   添字 >= end   … like_numbers[i] >= N と確定 (残した)
//   [start, end)  … 未確定
// 未確定が空 (start == end) になった時点が境界。残す側は end = cursor、捨てる側は
// start = cursor + 1 と、捨てる側にだけ +1 を付けるのがこの形の要点。両方に ±1 を付けると
// lo == hi + 1 で追い越して終わる別形式になり、どちらが答えか分からなくなる。
//
// なお N <= 919 = 最大の 326-like number なので答えは必ず存在し、start == len まで詰まって
// 添字アクセスが破綻することはない (存在しうる問題なら len との比較が要る)。

#[fastout]
fn main() {
    input! {
        n: usize,
    };

    let mut like_numbers = vec![];
    for x in 100..=919 {
        let hundred = x / 100;
        let ten = x / 10 % 10;
        let one = x % 10;
        if hundred * ten == one {
            like_numbers.push(x)
        }
    }

    let mut start = 0;
    let mut end = like_numbers.len();

    while start < end {
        let cursor = (start + end) / 2;
        if like_numbers[cursor] >= n {
            end = cursor;
        } else {
            start = cursor + 1;
        }
    }

    println!("{}", like_numbers[start]);
}

// alt: そもそも探索範囲が N..=919 と狭い (最悪 820 回) ので、候補列も二分探索も作らず
//      N から 1 ずつ上げて最初に条件を満たしたものを出せば済む。これが想定解で最短
// for x in n..=919 {
//     if (x / 100) * (x / 10 % 10) == x % 10 {
//         println!("{x}");
//         return;
//     }
// }

// alt: 手書き二分探索は標準ライブラリの partition_point (C++ の lower_bound 相当) で丸ごと
//      置き換えられる。述語には「捨てる側の条件」= like_numbers[cursor] < n をそのまま渡す。
//      返るのは述語が false になる最初の添字なので、それが N 以上で最小の要素の位置になる
// let i = like_numbers.partition_point(|&x| x < n);
// println!("{}", like_numbers[i]);

// alt: 候補は「百の位 a と十の位 b を決めれば一の位が a*b に定まる」ので、820 回の判定を
//      せず 100*a + 10*b + a*b を直接組み立てられる。a * b <= 9 が一の位に収まる条件。
//      a 昇順・b 昇順で回せば生成順がそのまま昇順になるのでソートも不要 (全 32 個)
// let like_numbers = (1..=9usize)
//     .cartesian_product(0..=9usize)
//     .filter(|&(a, b)| a * b <= 9)
//     .map(|(a, b)| 100 * a + 10 * b + a * b)
//     .collect_vec();

// alt: typical90 001 と同じ ok/ng 形式 (めぐる式)。ng = 「満たさないと確定」、ok = 「満たすと
//      確定」を配列の外側に番兵として置き、隣り合う (= 未確定が空) まで詰める。ループ本体から
//      ±1 が消えるため境界の付け方を間違えにくく、条件を逆向き (N 以下で最大など) にしたい
//      ときも ok/ng の初期値を入れ替えるだけで本体は書き換えずに済む。
//      ただし番兵 ng = -1 のために i64 が要り、Rust では添字のキャストが増える
// let mut ng: i64 = -1;
// let mut ok: i64 = like_numbers.len() as i64;
// while ok - ng > 1 {
//     let mid = (ok + ng) / 2;
//     if like_numbers[mid as usize] >= n {
//         ok = mid;
//     } else {
//         ng = mid;
//     }
// }
// println!("{}", like_numbers[ok as usize]);
