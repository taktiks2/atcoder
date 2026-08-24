use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [usize; m],
    };

    let mut b = vec![0; n];
    for ai in a {
        b[ai - 1] = 1;
    }

    let mut ans = vec![0; n];
    for i in (0..n).rev() {
        if b[i] == 1 {
            continue;
        } else {
            ans[i] = ans[i + 1] + 1;
        }
    }

    for ans_i in ans {
        println!("{}", ans_i)
    }
}
