use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        s: String,
    };

    let len = s.len();

    let mut chars = HashSet::new();

    for c in s.chars() {
        chars.insert(c);
    }

    let mut ans = "no";
    if len == chars.len() {
        ans = "yes";
    }

    println!("{}", ans);
}
