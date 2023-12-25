use proconio::input;

const WORDS: [&str; 5] = ["and", "not", "that", "the", "you"];

fn main() {
    input! {
        n: usize,
        w: [String; n],
    };

    for word in w {
        if WORDS.iter().any(|&item| item == word) {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
