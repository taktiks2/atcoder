use proconio::input;

fn main() {
    input! {
        s: String,
    };
    let (head, _) = s.split_at(s.len() - 1);
    let ans = format!("{}{}", head, '4');
    println!("{}", ans)
}
