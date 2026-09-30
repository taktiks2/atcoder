// algo: brute-force
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

// bit全探索
//
// M 個の集合それぞれを「選ぶ / 選ばない」の 2 択にすると、選び方は 2^M 通り。M <= 10 なので
// 高々 1024 通りしかなく、全部試して条件を満たすものを数えればよい。1 通りあたりの判定は
// 選んだ集合の要素を全部なめる O(N*M) = 100 程度で、合計でも 10^5 回に収まる。
//
// bit の i ビット目を「S_i を選ぶか」に対応させる。target[x] は選んだ集合のうち x を含む
// 個数で、全要素が 1 以上なら 1..=N を覆えている。個数ではなく bool でも十分だが、
// 「含む集合が少なくとも 1 個」をそのまま数で表した形。
//
// 問題は「1 個以上選ぶ」選び方を数えるが、ループは bit = 0 (何も選ばない) も含む。
// N >= 1 なので空の選び方は必ず target が全部 0 になって数えられず、除外の処理は要らない。
// N = 0 があり得る問題なら 1 から回す必要がある。
//
// a は Usize1 で 0-indexed にしておくと target の添字にそのまま使える。1-indexed のまま
// だと target を n+1 要素にして target[0] を判定から外す、といった一手間が増える。

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [[Usize1]; m],
    };

    let mut cnt = 0;
    for bit in 0..(1 << m) {
        let mut target: Vec<usize> = vec![0; n];
        for i in 0..m {
            if bit >> i & 1 == 1 {
                for &j in &a[i] {
                    target[j] += 1;
                }
            }
        }
        if target.iter().all(|&x| x > 0) {
            cnt += 1;
        }
    }

    println!("{cnt}");
}

// alt: 各集合を「含む要素のビットが立った整数」に前計算しておくと、選んだ集合の和集合は
//      OR を取るだけで求まり、覆えているかは (1 << N) - 1 との比較 1 回で済む。
//      判定が O(N*M) から O(M) に落ち、Vec の確保もループから消える。bit を二重に使う典型形
// let full = (1usize << n) - 1;
// let masks: Vec<usize> = a.iter().map(|s| s.iter().fold(0, |acc, &j| acc | 1 << j)).collect();
// let cnt = (0..1usize << m)
//     .filter(|&bit| (0..m).filter(|&i| bit >> i & 1 == 1).fold(0, |acc, i| acc | masks[i]) == full)
//     .count();

// alt: itertools の powerset で部分集合を直接列挙すると、ビット演算を一切書かずに済み最短。
//      「1..=N の全要素が、選んだどれかに含まれる」という問題文をそのまま all / any で書ける。
//      powerset は空集合も返すが、本体と同じ理由で数えられない。Vec を毎回作るので定数倍は重いが、M <= 10 なら問題ない
// let cnt = (0..m)
//     .powerset()
//     .filter(|sel| (0..n).all(|x| sel.iter().any(|&i| a[i].contains(&x))))
//     .count();
