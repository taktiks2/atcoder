use proconio::input;
use regex::Regex;

fn main() {
    input! {
        n: usize,
        q: usize,
        s: String,
        lr: [(i32, i32); q],
    };

    let re = Regex::new(r"[a-z]{2}").unwrap();

    for i in 0..q {
        let (l, r) = lr[i];
        let ss: &str = &s;
        let part = &ss[(l as usize)..=(r as usize)];

        println!("{}", re.find_iter(&part).count());
    }
}
