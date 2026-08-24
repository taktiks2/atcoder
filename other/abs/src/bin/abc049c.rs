use proconio::input;

fn main() {
    input! {
        mut s: String,
    };

    let mut ans = "NO";

    s = s.replace("eraser", "");
    s = s.replace("erase", "");
    s = s.replace("dreamer", "");
    s = s.replace("dream", "");

    if s.is_empty() {
        ans = "YES";
    }

    println!("{}", ans);
}
