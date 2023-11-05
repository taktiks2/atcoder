use proconio::input;

fn main() {
    input! {
        _: u32,
        s: String,
    };

    let mut ans = "No";

    if s.contains("ab") || s.contains("ba") {
        ans = "Yes";
    }

    println!("{}", ans);
}
