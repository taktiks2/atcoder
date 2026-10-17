#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: i64,
        t: i64,
        a: [i64; n],
    };

    let mut sub: Vec<i64> = vec![0; n + 1];
    sub[0] = a[0];

    let mut last_idx = 0usize;
    for (i, &ai) in a[1..].iter().enumerate() {
        if t > ai {
            sub[i + 1] = ai - a[i];
            last_idx = i + 1;
        }
    }

    if t - a[last_idx] > k {
        sub[n] = t - a[last_idx];
    }

    let mut ans = 0i64;
    for subi in sub {
        if subi - k > 0 {
            ans += subi / k;
        }
    }

    println!("{ans}");
}
