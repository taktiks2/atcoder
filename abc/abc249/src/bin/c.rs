// algo: brute-force, counting
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

// bit全探索
//
// 各 S_i を「選ぶ / 選ばない」の 2 択にすると選び方は 2^N 通り。N <= 15 なので高々
// 32768 通りで、全部試して「ちょうど K 個の文字列に現れる文字の種類数」の最大を取ればよい。
// 1 通りあたりの集計は選んだ文字列の文字を全部なめる O(N * 26) = 390 程度で、合計でも
// 32768 * 390 ≈ 1.3 * 10^7 回に収まる。
//
// 評価は「選び方ごと」に独立なので、map は bit ごとに作り直す。外に出して使い回すと前の
// 選び方のカウントが残り、サンプルは通っても値が膨らんで WA になる。
//
// map の値は厳密には「文字の出現回数」であって「その文字を含む文字列の個数」ではない。
// 両者が一致するのは制約「S_i に同じ文字は 2 個以上含まれない」のおかげ。同じ文字が
// 繰り返し得る問題だと、例えば "aa" 1 本で a が 2 と数えられて壊れる。その場合は S_i を
// 先に dedup するか、後述の alt のように「文字列ごとの所属」を bit で持つ必要がある。
//
// bit = 0 (何も選ばない) もループに含まれるが、そのとき map は空で 0 になるだけなので
// 最大値には影響しない。K >= 1 なので「0 個の文字列に現れる文字」を数えてしまう心配もない。

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        s: [Chars; n],
    };

    let mut cnt = 0usize;

    for bit in 0..(1 << n) {
        let mut map: HashMap<char, usize> = HashMap::new();
        for i in 0..n {
            if bit >> i & 1 == 1 {
                for &c in &s[i] {
                    *map.entry(c).or_insert(0) += 1;
                }
            }
        }
        cnt = cnt.max(map.values().filter(|&&x| x == k).count())
    }

    println!("{cnt}");
}

// alt: HashMap を [usize; 26] に置き換え、ループを map + max に畳んだ版。英小文字限定なので
//      添字で直接引けてハッシュ計算と 32768 回のヒープ確保が消える。可変の cnt も要らなくなり
//      「選び方ごとの値の最大」という問題の形がそのまま式に出る
// let ans = (0..1usize << n)
//     .map(|bit| {
//         let mut freq = [0usize; 26];
//         for i in (0..n).filter(|&i| bit >> i & 1 == 1) {
//             for &c in &s[i] {
//                 freq[(c as u8 - b'a') as usize] += 1;
//             }
//         }
//         freq.iter().filter(|&&x| x == k).count()
//     })
//     .max()
//     .unwrap();
// println!("{ans}");

// alt: 視点を逆にして「文字 c を含む文字列の集合」を owners[c] として bit で前計算する版。
//      選び方 bit のもとで c を含む文字列の個数は (owners[c] & bit).count_ones() の一発で出る。
//      1 通りあたり O(26) になり全体 O(2^N * 26) ≈ 8.5 * 10^5 と一桁以上速い。所属を
//      集合で持つので S_i に同じ文字が重複していても正しく、本解の前提に依存しない
// let mut owners = [0u32; 26];
// for (i, si) in s.iter().enumerate() {
//     for &c in si {
//         owners[(c as u8 - b'a') as usize] |= 1 << i;
//     }
// }
// let ans = (0..1u32 << n)
//     .map(|bit| owners.iter().filter(|&&o| (o & bit).count_ones() as usize == k).count())
//     .max()
//     .unwrap();
// println!("{ans}");
