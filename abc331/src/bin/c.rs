use itertools::Itertools;
use proconio::input;
use std::collections::BTreeMap;

fn main() {
    input! {
        n: usize,
        a: [i32; n],
    };

    let mut dup_map = BTreeMap::new();
    for &ai in &a {
        dup_map.entry(ai).or_insert(Vec::new()).push(ai);
    }

    let mut sum_map = BTreeMap::new();
    let mut sum = 0;
    for (key, arr) in dup_map.iter().sorted_by(|a, b| b.0.cmp(a.0)) {
        sum_map.insert(key, sum);
        sum += arr.iter().sum::<i32>();
    }

    let ans = a.iter().map(|ai| sum_map.get(ai).unwrap()).join(" ");
    println!("{}", ans)
}
