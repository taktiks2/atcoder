use proconio::input;

const TARGET: &str = "ABC";

fn main() {
    input! {
        mut s: String,
    };

    loop {
        if s.contains(TARGET) {
            s = s.replace(TARGET, "")
        } else {
            break;
        }
    }

    if s.len() != 0 {
        println!("{}", s);
    }
}
