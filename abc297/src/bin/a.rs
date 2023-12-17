use proconio::input;

fn main() {
    input! {
        n: usize,
        d: usize,
        t: [usize; n],
    };

    let mut x1 = 0;
    let mut ans: i32 = -1;
    for ti in t {
        if x1 == 0 {
            x1 = ti;
            continue;
        }

        if ti - x1 <= d {
            ans = ti as i32;
            break;
        }

        x1 = ti;
    }

    println!("{}", ans);
}
