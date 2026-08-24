use proconio::input;

fn main() {
    input! {
        n: usize,
        d: usize,
        t: [usize; n],
    };

    for ti in t.windows(2) {
        if ti[1] - ti[0] <= d {
            println!("{}", ti[1]);
            return;
        }
    }

    println!("-1");
}
