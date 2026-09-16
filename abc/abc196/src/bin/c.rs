// algo: enumeration, binary-search
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

// 1 以上 N 以下の整数のうち「桁数が偶数で、前半と後半が一致するもの」を数える問題。
// N <= 10^12 なので 1..=N を舐めるのは論外。x を探すのをやめて x を作る側に回る。
//
// 条件を満たす x は前半 k を決めた時点で「k を二回並べたもの」と一意に決まり、逆に
// k が違えば x も違う。つまり k <-> x は全単射で、数えるべきは x ではなく k の個数。
// 探索空間が N から「前半の候補」に一気に縮む。
//
// k の上限は N の制約から出る。N <= 10^12 は 13 桁なので、対象になる x は高々 12 桁、
// その前半 k は高々 6 桁 = 999999。候補は 10^6 個しかないので全部作れる。上限を多めに
// 取っても「N 以下か」の判定で落ちるだけなので害はないが、型は i64 が要る
// (k = 999999 のとき x = 999999999999 ≈ 10^12 で i32 は溢れる)。
//
// 後半の先頭 0 を気にしなくてよいのがこの作り方の利点。例えば 1001 は前半 10 / 後半 01
// で条件を満たさないが、k を整数として 1 から回す限り k 自身に先頭 0 は付かず、format!
// で並べた文字列は必ず「前半 = 後半 = k」になる。x の桁を直接見に行く実装だと、この
// 先頭 0 の扱いで間違えやすい。
//
// targets は昇順に並ぶ。k が増えると桁数は単調非減少で、同じ桁数の中では k について
// 単調増加だから (99 の次が 1010 のように、桁が増える境目でも逆転しない)。よってソート
// 不要でそのまま partition_point が使える。述語 x <= n は前半 true / 後半 false に
// 分かれ、返り値 = true の個数 = そのまま答え。
//
// 計算量は生成の O(10^6) が支配的で、二分探索の O(log) は誤差。手元の release ビルドで
// 50 ms 程度。出力は 1 行なので #[fastout] は本問では効いていない。

#[fastout]
fn main() {
    input! {
        n: i64,
    };

    let targets: Vec<i64> = (1..=999999)
        .map(|x| format!("{}{}", x, x).parse::<i64>().unwrap())
        .collect();

    let ans = targets.partition_point(|&x| x <= n);
    println!("{ans}");
}

// alt: そもそも二分探索は要らない。生成した時点で 10^6 個を触っているので、昇順性を
//      使って take_while で打ち切れば log すら不要。Vec を作らない分 8 MB の確保も消える。
//      「列挙して数える」という問題の形がそのまま出るので、実戦ではこちらが素直
// let ans = (1..=999999i64)
//     .map(|k| format!("{k}{k}").parse::<i64>().unwrap())
//     .take_while(|&x| x <= n)
//     .count();

// alt: 文字列を経由せず算術で組み立てる版。前半の桁数 d を外側に置けば 10^d が固定になり、
//      x = k * 10^d + k の掛け算一回で作れる。format! + parse の 10^6 回のアロケーションが
//      消えるので定数倍が一桁速い。桁数ごとに区切る形は「d 桁の前半は 10^(d-1) 以上」という
//      先頭 0 の禁止がループ範囲そのものになる点も読みやすい
// let mut ans = 0;
// let mut p = 10i64; // 前半が d 桁のときの 10^d
// for _d in 1..=6 {
//     for k in (p / 10)..p {
//         if k * p + k <= n {
//             ans += 1;
//         }
//     }
//     p *= 10;
// }

// alt: f(k) = 「k を二回並べた数」が k について単調増加なのだから、列挙せずに k 自体を
//      二分探索してもよい。O(log) で、テーブルを持たないのでメモリも定数。ok = 0 は
//      「条件を満たす k が一つも無い」ための番兵で、答えは 1..=ok の個数 = ok そのもの。
//      ng = 10^6 は f(10^6) = 10^13 > N なので必ず条件を満たさない
// let f = |k: i64| -> i64 {
//     let mut p = 10;
//     while p <= k {
//         p *= 10;
//     }
//     k * p + k
// };
// let mut ok = 0i64; // 番兵。f(k) <= n を満たす最大の k
// let mut ng = 1_000_000i64; // 番兵。f(10^6) は N を必ず超える
// while ng - ok > 1 {
//     let mid = (ok + ng) / 2;
//     if f(mid) <= n {
//         ok = mid;
//     } else {
//         ng = mid;
//     }
// }
// println!("{ok}");

// alt: 探索すら要らず O(桁数) の閉じた式で出る。N の桁数を len とすると、
//      len が奇数なら len 桁の x は一つも条件を満たせない (桁数が偶数でないと駄目) ので、
//      len-1 桁以下の x が全部答えに入り、前半は (len-1)/2 桁以下 = 10^((len-1)/2) - 1 個。
//      len が偶数なら m = len/2 として N を上位 m 桁 head と下位 m 桁 tail に割る。
//      k < head なら x は N より確実に小さく (桁数が足りない k も 1..head-1 に全部含まれる)、
//      k = head のときだけ tail >= head かどうかで決まる。境界の一個だけを比較で処理する形
// let s = n.to_string();
// let len = s.len();
// let ans = if len % 2 == 1 {
//     10i64.pow((len as u32 - 1) / 2) - 1
// } else {
//     let m = len / 2;
//     let head: i64 = s[..m].parse().unwrap();
//     let tail: i64 = s[m..].parse().unwrap();
//     (head - 1) + if tail >= head { 1 } else { 0 }
// };
