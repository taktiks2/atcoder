use proconio::input;

fn main() {
    input! {
        s: String,
    };

    for (i, c) in s.chars().enumerate() {
        if i == s.len() - 1 {
            print!("{}", c);
            break;
        }
        print!("{} ", c);
    }
}
