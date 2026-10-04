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
        q: usize,
        mut p: [i64; n],
        a: [i64; q],
    };

    let mut s: String = p.iter().map(|x| x.to_string()).join(" ");

    for ai in a {
        let pattern = ai.to_string();
        s = format!("{} {}", s.replace(&pattern, ""), ai);
    }

    let ans = s.split_whitespace().join(" ");

    println!("{ans}");
}
