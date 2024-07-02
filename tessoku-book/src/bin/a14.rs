use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
        b: [usize; n],
        c: [usize; n],
        d: [usize; n],
    };

    let mut p = vec![0; n * n + 1];
    for i in 1..=n {
        for j in 1..=n {
            p[n * (i - 1) + j] = a[i - 1] + b[j - 1]
        }
    }

    let mut q = vec![0; n * n + 1];
    for i in 1..=n {
        for j in 1..=n {
            q[n * (i - 1) + j] = c[i - 1] + d[j - 1]
        }
    }

    q.sort();

    // println!("{:?}", p);
    // println!("{:?}", q);

    for i in 1..=n * n {
        let target = k - p[i];
        let left = q.binary_search(&target);

        // println!("{:?}", left);
        match left {
            Ok(n) => {
                // println!("{}", n);
                if q[n] == target {
                    println!("Yes");
                    return;
                }
            }
            Err(_) => continue,
        }
    }
}
