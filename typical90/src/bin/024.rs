use proconio::input;

fn main() {
    input! {
        n: usize,
        k: i32,
        a: [i32; n],
        b: [i32; n],
    };

    let mut ans = "No";

    let mut diff = 0;
    for i in 0..n {
        diff += (a[i] - b[i]).abs()
    }


    if diff <= k && k % 2 == diff % 2 {
        ans = "Yes";
    }

    println!("{}", ans);
}
