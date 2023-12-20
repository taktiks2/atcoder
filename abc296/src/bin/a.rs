use proconio::input;

fn main() {
    input! {
        _: usize,
        s: String,
    };

    let mut ans = "Yes";
    for c2 in s.chars().collect::<Vec<char>>().windows(2) {
        if c2[0] == c2[1] {
            ans = "No";
        }
    }

    println!("{}", ans)
}
