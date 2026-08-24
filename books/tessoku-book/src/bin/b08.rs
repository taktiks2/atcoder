use proconio::input;

fn main() {
    input! {
        n: usize,
        z: [[usize; 2]; n],
        q: usize,
        e: [[usize; 4]; q]
    };

    const SIZE: usize = 1509;

    let mut plane = vec![vec![0; SIZE]; SIZE];

    for l in z {
        plane[l[1]][l[0]] += 1
    }

    let mut acc = vec![vec![0; SIZE]; SIZE];
    for i in 1..SIZE {
        for j in 1..SIZE {
            acc[i][j] = acc[i][j - 1] + plane[i][j]
        }
    }

    for i in 1..SIZE {
        for j in 1..SIZE {
            acc[j][i] = acc[j - 1][i] + acc[j][i]
        }
    }

    for l in e {
        let (a, b, c, d) = (l[0], l[1], l[2], l[3]);

        println!(
            "{}",
            acc[d][c] + acc[b - 1][a - 1] - acc[b - 1][c] - acc[d][a - 1]
        )
    }
}
