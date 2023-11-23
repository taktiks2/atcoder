use proconio::input;

// グラフ

fn main() {
    input! {
        n: usize,
        m: usize,
    };

    let mut groups = vec![vec![]; n];

    for _ in 0..m {
        input! {
            a: usize,
            b: usize,
        };

        groups[a - 1].push(b);
        groups[b - 1].push(a);
    }

    let mut cnt = 0;
    for (index, group) in groups.iter().enumerate() {
        let mut target = 0;
        for &member in group {
            if member < index + 1 {
                target += 1;
            }
        }
        if target == 1 {
            cnt += 1;
        }
    }

    println!("{}", cnt);
}
