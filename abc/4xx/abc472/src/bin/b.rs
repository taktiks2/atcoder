// algo: prefix-sum
use proconio::input;

fn main() {
    input! {
        n: usize,
        l: [i32; n],
    };

    let prefix_sum: Vec<i32> = l
        .iter()
        .scan(0, |left, x| {
            *left += x;
            Some(*left)
        })
        .collect();

    let mut min = 0;

    for i in 0..n - 1 {
        let abs = (prefix_sum[n - 1] - prefix_sum[i] * 2).abs();
        if i == 0 {
            min = abs
        }
        if abs < min {
            min = abs;
        }
    }

    println!("{}", min);
}

// alt: map + min で手動の min 管理と i == 0 の分岐が消える
// let total = prefix_sum[n - 1];
// let ans = prefix_sum[..n - 1]
//     .iter()
//     .map(|left| (total - left * 2).abs())
//     .min()
//     .unwrap();
// println!("{}", ans);

// alt: scan で逐次計算すれば累積和の配列自体が不要 (追加メモリ O(1))
// let total: i32 = l.iter().sum();
// let ans = l[..n - 1]
//     .iter()
//     .scan(0, |left, x| {
//         *left += x;
//         let sub = (total - *left * 2).abs();
//         Some(sub)
//     })
//     .min()
//     .unwrap();
// println!("{}", ans);
