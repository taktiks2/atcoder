use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        abcd: [(usize, usize, usize, usize); n]
    };

    let mut base = vec![vec![0; w + 2]; h + 2];

    for l in abcd {
        let (a, b, c, d) = l;
        base[a][b] += 1;
        base[c + 1][d + 1] += 1;
        base[a][d + 1] -= 1;
        base[c + 1][b] -= 1
    }

    let mut acc = vec![vec![0; w + 2]; h + 2];

    for i in 1..=h {
        for j in 1..=w {
            acc[i][j] = acc[i][j - 1] + base[i][j]
        }
    }

    for j in 1..=w {
        for i in 1..=h {
            acc[i][j] = acc[i - 1][j] + acc[i][j]
        }
    }

    for i in 1..=h {
        for j in 1..=w {
            if j >= 2 {
                print!(" ")
            }
            print!("{}", acc[i][j])
        }
        println!()
    }
}
