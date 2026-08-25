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

    // より良い解法
    // let total = pre_sum[n - 1];
    // let ans = pre_sum[..n - 1]
    //     .iter()
    //     .map(|left| (total - left * 2).abs())
    //     .min()
    //     .unwrap();
    // println!("{}", ans);

    // もっと良い解法
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
}
