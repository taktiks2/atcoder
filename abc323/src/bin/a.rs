use proconio::input;

fn main() {
    input! {
        s: String,
    };

    let mut ans = "Yes";
    for (i, c) in s.chars().enumerate() {
        if i % 2 == 1 && c != '0' {
            ans = "No";
        }
    }

    println!("{}", ans);
}
