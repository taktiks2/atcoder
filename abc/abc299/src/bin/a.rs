use proconio::input;

fn main() {
    input! {
        _: usize,
        s: String,
    };

    let mut ans = "out";
    let mut inside = false;

    for c in s.chars() {
        if inside && c == '*' {
            ans = "in";
            break;
        }
        if inside && c == '|' {
            break;
        }
        if c == '|' {
            inside = true;
        }
    }

    println!("{}", ans);
}
