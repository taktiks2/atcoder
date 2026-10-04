// algo: sort
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

// 長さ K の区間を 1 回だけソートして、A 全体を昇順にできるかを判定する問題。
// 開始位置を全部試して毎回ソートすると O(N K log K) かかり、N = 2*10^5, K = 10^5 で
// 約 3*10^11 になって TLE。
//
// 最終形は sorted (A をソートした列) しかありえない。区間の外は操作で動かないので、
// 「区間外の要素がすでに sorted と一致していること」が必要。逆にこれが成り立てば、
// 区間内の要素の多重集合は sorted の同じ区間と一致する (全体から外側を除いた残りなので)。
// だからソートすれば一致する。つまり十分条件でもある。
//
// sorted と先頭から一致する長さを p、末尾から一致する長さを s とすると、
// 外側を [0, i) と [i+K, N) に収められる開始位置 i が存在する ⇔ p + s >= N - K。
// コード中の cnt が p + s。A がすでに昇順だと p = s = N で二重に数えるが、
// >= で判定するので問題ない。
//
// 「隣同士が昇順か」だけを見るのは誤り。たとえば A = (2, 3, 1), K = 2 は左 2 つが
// 昇順だが、どちらの区間をソートしても (1, 2, 3) にならない。比べる相手は隣の要素では
// なく sorted。K < N なので n - k はアンダーフローしない。
// 全体は O(N log N)。

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    };

    let mut sorted = a.clone();
    sorted.sort_unstable();

    let mut judge: Vec<bool> = vec![false; n];

    let mut cnt = 0usize;
    let mut ok = true;
    for (i, (&a, b)) in a.iter().zip(sorted).enumerate() {
        if a == b {
            judge[i] = true;
            if ok {
                cnt += 1;
            }
        } else {
            ok = false;
        }
    }

    judge.reverse();

    for x in judge {
        if x {
            cnt += 1;
        } else {
            break;
        }
    }

    let sub = n - k;
    if sub <= cnt {
        println!("Yes");
    } else {
        println!("No");
    }
}

// alt: judge 配列を作らず、前後からの一致長を take_while().count() で直接数える。
//      p と s が別の変数になるので「p + s >= N - K」という判定の形がそのまま見え、
//      ok フラグや reverse による状態管理も消えて短い
// let mut sorted = a.clone();
// sorted.sort_unstable();
// let pre = a.iter().zip(&sorted).take_while(|(x, y)| x == y).count();
// let suf = a.iter().rev().zip(sorted.iter().rev()).take_while(|(x, y)| x == y).count();
// println!("{}", if pre + suf >= n - k { "Yes" } else { "No" });
