use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    };

    // O(N)
    for c in s {
        if c == 'A' { print!("A") } else { print!(".") }
    }

    // より良い開放
    // let s: String = s
    //     .iter()
    //     .map(|&c| if c == 'A' { 'A' } else { '.' })
    //     .collect();
    // println!("{}", s);
    //
    // input! {
    //     s: String,
    // }
    // let s = s.replace(|c| c != 'A', ".");
    // println!("{}", s);
}
