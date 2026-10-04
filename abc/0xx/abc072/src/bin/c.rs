// algo: counting
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

// 各 a_i を -1 / 0 / +1 して、同じ値になる要素の個数を最大化する問題。
// 言い換えると「X を一つ決め、|a_i - X| <= 1 を満たす i の個数を最大化」。
//
// 素朴に X を全部試して毎回 N 個を数えると、X の候補 10^5 × N = 10^5 で 10^10 回になり
// 間に合わない。向きを逆にして、各 a_i から「自分が一票入れられる X」(a_i-1, a_i, a_i+1)
// へ票を配る。票の合計は 3N = 3*10^5 で、最多得票数がそのまま答え。X ごとに数えるのではなく
// a_i ごとに配るので、計算量は O(N)。
//
// 一つの a_i が同じ X に二票入れることはない (a_i-1, a_i, a_i+1 は全部違う) ので、
// X の得票数は「X に合わせられる要素の個数」と一致する。
//
// 落とし穴は a_i = 0 のときの a_i - 1。usize なので debug ビルドでは overflow で panic、
// release では usize::MAX に巻き戻る。checked_sub で X = -1 への票を捨てているが、
// これで答えは変わらない。X = -1 に票を入れられるのは a_i = 0 だけで、その要素は X = 0 にも
// 必ず票を入れているから、X = 0 の得票数は X = -1 以上になる。
//
// 個数は N <= 10^5 が上限なので usize で溢れる心配はない。

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };

    let mut map: HashMap<usize, usize> = HashMap::new();

    for ai in a {
        if let Some(result) = ai.checked_sub(1) {
            *map.entry(result).or_insert(0) += 1;
        }
        *map.entry(ai).or_insert(0) += 1;
        *map.entry(ai + 1).or_insert(0) += 1;
    }

    let max = map.values().max().unwrap();

    println!("{max}");
}

// alt: a_i < 10^5 と値域が小さいので、HashMap をやめて Vec を添字で数える。ハッシュを
//      計算しない分だけ定数倍が速い。saturating_sub にすれば a_i = 0 のとき範囲が 0..=1 に
//      縮むだけなので、checked_sub の分岐も消える。X = 10^5 の票まで入るので長さは 10^5 + 2
// let mut cnt = vec![0usize; 100_002];
// for &ai in &a {
//     for x in ai.saturating_sub(1)..=ai + 1 {
//         cnt[x] += 1;
//     }
// }
// println!("{}", cnt.iter().max().unwrap());

// alt: 票を配る代わりに、値ごとの度数を一度だけ数え、幅 3 の窓の合計を見る。windows(3) で
//      「X-1, X, X+1 の度数の和」がそのまま書けて意図が読みやすい。度数を 1 つずらして
//      freq[a_i + 1] に置けば、X = 0 を中心にした窓が左端から始まる
// let mut freq = vec![0usize; 100_002];
// for &ai in &a {
//     freq[ai + 1] += 1;
// }
// let ans = freq.windows(3).map(|w| w.iter().sum::<usize>()).max().unwrap();
// println!("{ans}");

// alt: 値域が大きくて配列を取れない場合の形。ソートしたあと「最大値 - 最小値 <= 2」の区間を
//      尺取りで伸ばしていく。O(N log N) だが、値域に依存しない
// let mut a = a;
// a.sort_unstable();
// let mut l = 0;
// let mut ans = 0;
// for r in 0..n {
//     while a[r] - a[l] > 2 {
//         l += 1;
//     }
//     ans = ans.max(r - l + 1);
// }
// println!("{ans}");
