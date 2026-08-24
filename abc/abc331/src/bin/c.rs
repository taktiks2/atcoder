use itertools::Itertools;
use proconio::input;
use std::collections::BTreeMap;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };

    let mut dup_map = BTreeMap::new();
    for &ai in &a {
        dup_map.entry(ai).or_insert(Vec::new()).push(ai);
    }

    let mut sum_map = BTreeMap::new();
    let mut sum = 0;
    for (key, arr) in dup_map.iter().sorted_by(|a, b| b.0.cmp(a.0)) {
        sum_map.insert(key, sum);
        sum += arr.iter().sum::<usize>();
    }

    let ans = a.iter().map(|ai| sum_map.get(ai).unwrap()).join(" ");
    println!("{}", ans)
}

// use proconio::input;
// use std::collections::BTreeMap;
//
// fn main() {
//     input! {
//         n: usize,
//         a: [usize; n],
//     };
//
//     let mut b = a.clone();
//
//     b.sort_by(|a, b| b.cmp(a));
//
//     let mut sum_map = BTreeMap::new();
//     let mut sum = 0;
//     let mut sum_before = 0;
//
//     for i in 0..n {
//         if i == 0 {
//             sum_map.insert(b[i], sum_before);
//             sum += b[i];
//             continue;
//         }
//         if b[i] != b[i - 1] {
//             sum_before = sum;
//         }
//         sum_map.insert(b[i], sum_before);
//         sum += b[i];
//     }
//
//     for (i, ae) in a.iter().enumerate() {
//         let ans = sum_map.get(ae).unwrap();
//         if i == n - 1 {
//             print!("{}", ans);
//             break;
//         }
//         print!("{} ", ans)
//     }
// }
