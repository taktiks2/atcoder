use proconio::input;

fn main() {
    input! {
        s: String,
    };

    let chars = ['a', 'i', 'u', 'e', 'o'];

    let ans: String = s.chars().filter(|&c| !chars.contains(&c)).collect();

    println!("{}", ans);
}
