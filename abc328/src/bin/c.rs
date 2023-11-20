use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        s: String,
        lr: [(i32, i32); q],
    };

    let mut ss = vec![];
    for c in s.chars() {
        ss.push(c);
    }

    let mut a = vec![];
    for i in 0..n {
        if i > 0 && ss[i] == ss[i - 1] {
            a.push(1);
        } else {
            a.push(0);
        }
    }

    let mut b = vec![0; a.len()];
    for i in 0..a.len() {
        if i == 0 {
            b[i] = a[i];
            continue;
        }
        b[i] = b[i - 1] + a[i];
    }

    for (l, r) in lr {
        println!("{}", b[r as usize - 1] - b[l as usize - 1]);
    }
}
