use proconio::input;

fn main() {
    input! {
        n: i32,
        a: i32,
    };

    let mut ans = "No";

    if n % 500 <= a {
        ans = "Yes";
    }

    println!("{}", ans)
}
