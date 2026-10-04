// algo: brute-force
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

// 4 桁の暗証番号のうち「o の数字はすべて 1 回以上使い、x の数字は 1 回も使わない」ものを
// 数える問題。入力は長さ 10 の文字列 S だけで、制約にパラメータが無い。候補は 0000〜9999 の
// 10^4 個しかないので、全部作って条件を判定すれば済む。1 候補あたり 4 桁を見るだけなので
// 4×10^4 回程度で終わる。
//
// 判定は 2 つの条件に分かれる。
// - x の数字が 1 つでも出てきたら不可。桁を見た瞬間に分かる
// - o の数字がすべて使われたか。全桁を見終わるまで分からないので、使った o を memo 上で
//   '@' に塗り、最後に 'o' が残っていないかで判定する
// memo を候補ごとに s.clone() で作り直すのが要点。使い回すと前の候補で塗った '@' が残り、
// 「一度でも使われた o」を数えてしまう。
//
// 先頭 0 も許される (0000 も候補) ので、format!("{:04}") で 0 埋めすること。"{}" で
// 文字列にすると 7 が "7" になり、0 を 3 個使ったことにならない。入力例 1 は 0 が o
// なので、ここを間違えると数が変わる。
//
// 冒頭の「o が 5 個以上なら 0」は正しいが、なくても答えは変わらない。4 桁では 5 種類以上の
// 数字を使えず、ループ側でも必ず 0 になるから。単なる早期 return。
// 出力は 1 行なので #[fastout] は本問では効いていない。

#[fastout]
fn main() {
    input! {
        s: Chars,
    };

    if s.iter().filter(|&&c| c == 'o').count() > 4 {
        println!("0");
        return;
    }

    let mut cnt = 0usize;

    for i in 0..=9999 {
        let secret = format!("{:04}", i);
        let mut ok = true;
        let mut memo = s.clone();
        for c in secret.chars() {
            let num = c.to_digit(10).unwrap() as usize;
            match s[num] {
                'o' => {
                    memo[num] = '@';
                }
                'x' => {
                    ok = false;
                }
                _ => {}
            }
        }
        if ok && memo.iter().filter(|&&c| c == 'o').count() == 0 {
            cnt += 1;
        }
    }

    println!("{cnt}");
}

// alt: 文字列と clone をやめて、10 ビットのマスクで判定する。o の集合を must、x の集合を
//      ban、候補で使った数字の集合を used として持てば、条件は
//      「used ⊇ must かつ used ∩ ban = ∅」のビット演算 2 回になる。候補ごとの format! と
//      Vec の clone (計 10^4 回のアロケーション) が消え、集合の包含という問題の形が式に
//      そのまま出る
// let mask = |ch: char| (0..10).filter(|&d| s[d] == ch).fold(0u32, |m, d| m | 1 << d);
// let (must, ban) = (mask('o'), mask('x'));
// let ans = (0..10000u32)
//     .filter(|&p| {
//         let used = (0..4).fold(0u32, |m, k| m | 1 << (p / 10u32.pow(k) % 10));
//         used & must == must && used & ban == 0
//     })
//     .count();
// println!("{ans}");

// alt: 包除原理で O(2^|o|) の閉じた式にする。x を除いた 10 - |x| 種類の数字で 4 桁を
//      作り、そこから「o のうち集合 T の数字を 1 回も使わない」ものを (-1)^|T| 倍で
//      足し引きする。T を使わない列は (10 - |x| - |T|)^4 通りなので
//        答え = Σ_{T ⊆ o} (-1)^|T| (10 - |x| - |T|)^4
//      o が 5 個以上でも和が自然に 0 になり、早期 return は要らない。符号付きの和なので
//      i64 で持つ。o + x <= 10 なので底は負にならない
// let o = s.iter().filter(|&&c| c == 'o').count() as u32;
// let x = s.iter().filter(|&&c| c == 'x').count() as i64;
// let ans: i64 = (0..1u32 << o)
//     .map(|t| {
//         let k = t.count_ones() as i64;
//         let sign = if k % 2 == 0 { 1 } else { -1 };
//         sign * (10 - x - k).pow(4)
//     })
//     .sum();
// println!("{ans}");
