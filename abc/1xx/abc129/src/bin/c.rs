// algo: dp
// solve: hint 2026-10-27
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

const MOD: i64 = 1_000_000_007;

// N 段の階段を 1 段か 2 段ずつ上り、壊れた段 a_i を踏まずに N 段目へ行く方法を数える問題。
// 上り方を全部並べると、壊れた段が無くても N 段目までの通り数はフィボナッチ数になり、
// N <= 10^5 では桁数が 2 万を超える。列挙は論外で、mod を取りながら数だけを持ち回す。
//
// i 段目への行き方は「最後の一歩」で重複なく 2 つに分かれる。最後が +1 なら直前は i-1 段目、
// +2 なら i-2 段目。よって dp[i] = dp[i-1] + dp[i-2] で、壊れた段は 0 通りとして扱う。
//
// 落とし穴: 「i-1 段目からは +1 と +2 の 2 通りに進めるから dp[i] = dp[i-1] * 2」は誤り。
// +2 を選ぶと i+1 段目に着くので、i 段目への入り方は増えない。×2 が正解と一致するのは
// dp[i-1] == dp[i-2] のときだけ。sample 1（3 段目が壊れている）はたまたまそうなって通るが、
// sample 3 は 1 段目が壊れているので dp = 1, 1, 2 の次で 3 にならず 4 になり、WA になる。
//
// Usize1 で受けているので dp[i] は (i+1) 段目を表し、0 段目（地面、1 通り）は配列に無い。
// i < 2 だけ場合分けしているのはこのため。壊れた段は -1 で印を付け、0 通り（到達不能）と
// 区別している。max(last, second_last) は片方が -1 のときに、もう片方を選ぶための書き方。
// 壊れた段が 2 つ連続したら、その先へは絶対に行けないので 0 を出して打ち切る。
//
// 型: 足すのは MOD 未満の値 2 つで最大 ≈ 2.0*10^9。i32 の上限 2.1*10^9 にぎりぎり収まるが、
// 余裕が無いので i64 にしている。計算量は O(N)。

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [Usize1; m],
    };

    let mut dp = vec![0i64; n];
    for ai in a {
        dp[ai] = -1;
    }

    for i in 0..n {
        if i < 2 && dp[i] >= 0 {
            dp[i] = if i == 0 {
                1
            } else if dp[i - 1] > 0 {
                2
            } else {
                1
            }
        } else if i >= 2 && dp[i] >= 0 {
            let last = dp[i - 1];
            let second_last = dp[i - 2];
            if last >= 0 && second_last >= 0 {
                dp[i] = (last + second_last) % MOD;
            } else if last >= 0 || second_last >= 0 {
                dp[i] = last.max(second_last);
            } else {
                println!("0");
                return;
            }
        }
    }

    println!("{}", dp[n - 1]);
}

// alt: 構造はそのまま（dp の長さ n・-1 の印・ループの形）で、場合分けを足し算 1 本に畳んだ版
//      - 壊れた段は `.max(0)` で 0 通りとして読むので、last / second_last の 3 分岐が消える
//      - 0 段目（地面）の 1 通りは i < 2 のときに定数で補うので、i < 2 の if ブロックが消える
//      - 壊れた段が連続すると自然に 0 が伝わるので、途中の println!("0"); return; が不要になる
// for i in 0..n {
//     if dp[i] < 0 {
//         continue;
//     }
//     let one = if i >= 1 { dp[i - 1].max(0) } else { 1 };
//     let two = match i {
//         0 => 0,
//         1 => 1,
//         _ => dp[i - 2].max(0),
//     };
//     dp[i] = (one + two) % MOD;
// }

// alt: 添字 = 段の番号にそろえた定番形。dp の長さを n+1 にして dp[0] = 1（地面）を置き、
//      壊れた段は bool 配列で持つ。Usize1 によるずれも -1 の印も無いので、漸化式
//      dp[i] = dp[i-1] + dp[i-2] がそのままコードになり、読み間違いが起きにくい
// input! {
//     n: usize,
//     m: usize,
//     a: [usize; m],
// };
//
// let mut broken = vec![false; n + 1];
// for ai in a {
//     broken[ai] = true;
// }
//
// let mut dp = vec![0i64; n + 1];
// dp[0] = 1;
// for i in 1..=n {
//     if broken[i] {
//         continue;
//     }
//     dp[i] = dp[i - 1];
//     if i >= 2 {
//         dp[i] = (dp[i] + dp[i - 2]) % MOD;
//     }
// }
//
// println!("{}", dp[n]);
