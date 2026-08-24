use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    };

    let mut ans = a / b;
    if a % b != 0 {
        ans += 1;
    }

    println!("{}", ans)
}
