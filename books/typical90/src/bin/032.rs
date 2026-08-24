use itertools::Itertools;
use proconio::input;

// 順列全探索

fn main() {
    input! {
        n: usize,
        a: [[i32; n]; n],
        m: usize,
    };

    if n == 1 {
        println!("{}", a[0][0]);
        return;
    }

    let mut kenaku = vec![vec![false; n]; n];

    for _ in 0..m {
        input! {
            x: usize,
            y: usize,
        };

        kenaku[y - 1][x - 1] = true;
        kenaku[x - 1][y - 1] = true;
    }

    let mut ans = 1_000_000;
    for p in (0..n).permutations(n) {
        let mut ttl = 0;
        for s in 0..n - 1 {
            if kenaku[p[s]][p[s + 1]] {
                ttl = 0;
                break;
            }
            ttl += a[p[s]][s];
            if s == n - 2 {
                ttl += a[p[s + 1]][s + 1];
            }
        }
        if ttl > 0 {
            ans = ans.min(ttl);
        }
    }
    if ans == 1_000_000 {
        ans = -1;
    }
    println!("{}", ans);
}
