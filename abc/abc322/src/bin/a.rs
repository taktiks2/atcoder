use proconio::input;

fn main() {
    input! {
        _: usize,
        s: String,
    };

    match s.find("ABC") {
        Some(i) => println!("{}", i + 1),
        None => println!("-1"),
    }
}
