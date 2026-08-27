// algo: string, counting
use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [String; n],
    };

    let lowercases: Vec<String> = s.iter().map(|c| c.to_lowercase()).collect();

    let mut unique_lowercases = lowercases.clone();

    unique_lowercases.sort();
    unique_lowercases.dedup();

    let ans = unique_lowercases
        .iter()
        .map(|ulc| lowercases.iter().filter(|lc| *ulc == **lc).count())
        .max()
        .unwrap();

    println!("{}", ans);
}

// alt: HashMap の entry API で度数集計 — O(N^2) → O(N)、カウントの定番パターン
// use std::collections::HashMap;
// let mut counts: HashMap<String, usize> = HashMap::new();
// for x in &s {
//     *counts.entry(x.to_lowercase()).or_insert(0) += 1;
// }
// println!("{}", counts.values().max().unwrap());

// alt: itertools の counts() なら集計が 1 式で最短 (同じく O(N))
// use itertools::Itertools;
// let counts = s.iter().map(|x| x.to_lowercase()).counts();
// println!("{}", counts.values().max().unwrap());
