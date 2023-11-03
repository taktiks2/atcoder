use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        m: [[u32; w]; h],
    };

    let mut ans = vec![vec![0; w]; h];

    for hi in 0..h {
        let row_sum: u32 = m[hi].iter().sum();

        for wj in 0..w {
            let mut ttl = 0;

            let col_sum = m.iter().map(|row| row[wj]).sum::<u32>();
            ttl += col_sum + row_sum - m[hi][wj];

            ans[hi][wj] = ttl;
        }
    }

    for row in ans {
        for (i, val) in row.iter().enumerate() {
            if i == row.len() - 1 {
                print!("{}", val);
            } else {
                print!("{} ", val);
            }
        }
        println!();
    }
}
