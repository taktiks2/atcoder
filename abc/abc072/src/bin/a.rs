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
        a: [usize; n],
    };

    let mut map: HashMap<i64, usize> = HashMap::new();

    for ai in a {
        *map.entry((ai - 1) as i64).or_insert(0) += 1;
        *map.entry((ai) as i64).or_insert(0) += 1;
        *map.entry((ai + 1) as i64).or_insert(0) += 1;
    }

    let max = map.values().max().unwrap();

    println!("{max}");
}
