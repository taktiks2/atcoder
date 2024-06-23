use proconio::input;

fn main() {
    input! {
        n: usize,
        abcd: [(usize, usize, usize, usize); n]
    };

    const SIZE: usize = 1501;

    let mut acc = vec![vec![0; SIZE]; SIZE];
    for (a, b, c, d) in abcd {
        acc[a][b] += 1;
        acc[c][d] += 1;
        acc[a][d] -= 1;
        acc[c][b] -= 1;
    }

    for i in 0..SIZE {
        for j in 1..SIZE {
            acc[i][j] += acc[i][j - 1]
        }
    }

    for j in 0..SIZE {
        for i in 1..SIZE {
            acc[i][j] += acc[i - 1][j]
        }
    }

    let mut cnt = 0;
    for i in 0..SIZE - 1 {
        for j in 0..SIZE - 1 {
            if acc[i][j] > 0 {
                cnt += 1
            }
        }
    }

    println!("{}", cnt)
}
