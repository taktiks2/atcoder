use proconio::input;
use regex::Regex;

fn main() {
    input! {
        a: usize,
        _: i32,
        s: String,
    };

    let re = Regex::new(r"^\d+$").unwrap();
    let mut new_s = String::new();

    for (i, c) in s.chars().enumerate() {
        if i == a && c != '-' {
            println!("No");
            return;
        }
        if i != a {
            new_s.push(c);
        }
    }

    if re.is_match(&new_s) {
        println!("Yes");
    } else {
        println!("No");
    }
}
