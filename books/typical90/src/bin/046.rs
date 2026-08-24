use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        b: [usize; n],
        c: [usize; n],
    };

    let mut a_remains = [0; 46];
    let mut b_remains = [0; 46];
    let mut c_remains = [0; 46];
    for i in 0..n {
        a_remains[a[i] % 46] += 1;
        b_remains[b[i] % 46] += 1;
        c_remains[c[i] % 46] += 1;
    }

    let mut ans = 0usize;
    for i in 0..46 {
        for j in 0..46 {
            for k in 0..46 {
                if (i + j + k) % 46 == 0 {
                    ans += a_remains[i] * b_remains[j] * c_remains[k];
                }
            }
        }
    }

    println!("{}", ans);
}
