use proconio::input;

// 二分探索

fn main() {
    input! {
        n: i32,
        mut a: [i32; n],
        q: i32,
        b: [i32; q],
    };

    if a.len() == 1 {
        for target in &b {
            println!("{}", (a[0] - target).abs());
        }
        return;
    }

    a.sort();

    for target in b {
        let mut s = 0;
        let mut m = a.len() / 2;
        let mut e = a.len() - 1;

        loop {
            if target == a[m] {
                println!("0");
                break;
            }

            if e - s == 1 {
                let diff_s = (a[s] - target).abs();
                let diff_e = (a[e] - target).abs();
                if diff_s > diff_e {
                    println!("{}", diff_e);
                } else {
                    println!("{}", diff_s);
                }
                break;
            }

            if target > a[m] {
                s = m;
            } else {
                e = m;
            }
            m = (s + e) / 2;
        }
    }
}
