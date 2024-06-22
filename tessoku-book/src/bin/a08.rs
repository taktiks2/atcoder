use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        x: [[usize; w]; h],
        q: usize,
        abcd: [[usize; 4]; q]
    };

    let mut acc = vec![vec![0; w + 2]; h + 2];

    // 横方向のacc
    for hi in 1..=h {
        for wj in 1..=w {
            acc[hi][wj] = acc[hi][wj - 1] + x[hi - 1][wj - 1]
        }
    }

    // 縦方向のacc
    for wi in 1..=w {
        for hj in 1..=h {
            acc[hj][wi] = acc[hj - 1][wi] + acc[hj][wi]
        }
    }

    // 総和を計算
    for l in abcd {
        let a = l[0];
        let b = l[1];
        let c = l[2];
        let d = l[3];
        println!(
            "{}",
            acc[c][d] + acc[a - 1][b - 1] - acc[c][b - 1] - acc[a - 1][d]
        )
    }
}
