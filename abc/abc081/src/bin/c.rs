// algo: counting, sort, greedy
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

// N 個の値を K 種類以下にするための最小の書き換え回数を求める問題。
// 書き換えた後の値は残す種類のどれかにすればよいので、コストは「消す種類の出現回数の合計」
// だけで決まる。したがって、出現回数の多い K 種類を残して、それ以外を全部書き換えるのが最適。
// 残す種類を 1 つ回数の少ないものに替えると、書き換えが増えることはあっても減ることはない。
//
// 素朴に「どの K 種類を残すか」を全部試すと C(種類数, K) 通りになる。種類数は最大
// N = 2*10^5 なので論外。出現回数のソートだけで決まり、O(N log N) で済む。
//
// 実装では、昇順ソートした回数の先頭 (種類数 - K) 個を足している。K >= 種類数 のときに
// v.len() - k を計算すると usize がアンダーフローして panic する (release ではラップして
// 範囲外の添字になる)。そのため分岐で 0 を返している。サンプル 2 (N = K = 4, 種類数 2) が
// ちょうどこのケース。
//
// 答えは高々 N = 2*10^5 なので usize で十分。

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    };

    let mut map: HashMap<usize, usize> = HashMap::new();

    for ai in a {
        *map.entry(ai).or_insert(0) += 1;
    }

    let mut v: Vec<usize> = map.values().cloned().collect();
    v.sort_unstable();

    let ans = if k >= v.len() {
        0
    } else {
        let sub = v.len() - k;
        v[0..sub].iter().sum()
    };

    println!("{ans}");
}

// alt: A_i <= N という制約を使い、HashMap の代わりに長さ N+1 の配列で数える。ハッシュの
//      定数倍が消える。降順ソートして先頭 K 個を飛ばせば残りが消す分になる。cnt の長さは
//      N+1 > K なので cnt[k..] は必ず有効で、種類数 <= K の分岐も要らない (0 の要素は和に効かない)
// let mut cnt = vec![0usize; n + 1];
// for &x in &a {
//     cnt[x] += 1;
// }
// cnt.sort_unstable_by(|x, y| y.cmp(x));
// let ans: usize = cnt[k..].iter().sum();

// alt: itertools の counts で度数を 1 行で集計する。回数を昇順に並べて rev し、上位 K 個を
//      skip した残りを足す。skip は要素が足りなくても空になるだけなので、アンダーフローの
//      分岐が消えて最短になる
// let ans: usize = a.iter().counts().into_values().sorted_unstable().rev().skip(k).sum();
