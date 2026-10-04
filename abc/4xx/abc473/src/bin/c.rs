// algo: counting
use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        _: usize,
        a: [usize; n],
    };

    let mut class = HashMap::new();
    for x in a {
        *class.entry(x).or_insert(0) += 1;
    }

    let numbers: Vec<i32> = class.values().cloned().collect();
    let max_num = class.values().cloned().max().unwrap();

    let ans = numbers.iter().filter(|&&x| x >= max_num - 1).count();

    println!("{ans}");
}

// alt: クラス番号が 1..=K なので Vec を添字カウンタにできる — ハッシュ不要の O(N + K)。
//      0 人のクラスも数に入るので、「K <= N より 0 人クラスは条件を満たさない」という
//      制約への依存が消えて素直に読める
// input! { n: usize, k: usize, a: [usize; n] };
// let mut cnt = vec![0usize; k + 1];
// for x in a {
//     cnt[x] += 1;
// }
// let max = *cnt[1..].iter().max().unwrap();
// let ans = cnt[1..].iter().filter(|&&c| c + 1 >= max).count();
// println!("{ans}");

// alt: values() を 2 回回せば numbers への collect が不要 (追加メモリ O(1))。
//      条件を c + 1 >= max と書くと max - 1 のアンダーフローを気にせず usize で通せる
// let max = *class.values().max().unwrap();
// let ans = class.values().filter(|&&c| c + 1 >= max).count();
// println!("{ans}");
