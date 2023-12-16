use proconio::input;

fn main() {
    input! {
        n: usize,
        p: usize,
        q: usize,
        d: [usize; n],
    };

    let min_d = d.iter().min().unwrap();
    let mut ans = p;
    if min_d + q <= p {
        ans = min_d + q;
    }
    println!("{}", ans);
}
