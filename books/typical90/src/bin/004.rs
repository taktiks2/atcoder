use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        m: [[u32; w]; h],
    };

    let mut ans = vec![vec![0; w]; h];

    let row_sums: Vec<u32> = m.iter().map(|row| row.iter().sum()).collect();
    let col_sums: Vec<u32> = (0..w).map(|index| m.iter().map(|row| row[index]).sum()).collect();

    for hi in 0..h {
        for wi in 0..w {
            ans[hi][wi] = row_sums[hi] + col_sums[wi] - m[hi][wi];
        }
    }

    for row in ans {
        println!("{}", row.iter().map(|num| num.to_string()).collect::<Vec<String>>().join(" "));
    }
}
