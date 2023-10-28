use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        ss: Chars,
    };

    let mut c = 0;

    for s in ss {
        if s == '1' {
            c += 1;
        }
    }

    println!("{}", c);
}
