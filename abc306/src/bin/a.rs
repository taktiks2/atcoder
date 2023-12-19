use proconio::input;

fn main() {
    input! {
        _: usize,
        s: String,
    };

    for c in s.chars() {
        print!("{}{}", c, c)
    }
}
