// algo: string
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
}

// alt: iterator + collect で宣言的に書ける
// let s: String = s
//     .iter()
//     .map(|&c| if c == 'A' { 'A' } else { '.' })
//     .collect();
// println!("{}", s);

// alt: String で受けて replace するのが最短
// input! {
//     s: String,
// }
// let s = s.replace(|c| c != 'A', ".");
// println!("{}", s);
