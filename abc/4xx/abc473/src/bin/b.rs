// algo: counting
use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };

    let mut counts = HashMap::new();

    for x in a {
        *counts.entry(x).or_insert(0) += 1;
    }

    let mut ans = 0;
    for (num, count) in counts {
        if count % 2 != 0 {
            ans += num;
        }
    }

    println!("{ans}");
}

// alt: A_i <= 100 なので固定長配列で数えられる — ハッシュ不要で定数倍が軽い O(N + 100)
// let mut cnt = [0usize; 101];
// for x in a {
//     cnt[x] += 1;
// }
// let ans: usize = (1..=100).filter(|&v| cnt[v] % 2 == 1).sum();
// println!("{ans}");

// alt: HashSet のトグルで「2 枚揃ったら食べる」操作をそのまま表現 — 集計と偶奇判定が要らない
// use std::collections::HashSet;
// let mut rest = HashSet::new();
// for x in a {
//     if !rest.remove(&x) {
//         rest.insert(x);
//     }
// }
// println!("{}", rest.iter().sum::<usize>());
