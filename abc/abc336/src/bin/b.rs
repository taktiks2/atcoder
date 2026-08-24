use proconio::input;

fn main() {
    input! {
        n: usize,
    };

    let binary_str = format!("{:b}", n);
    let mut cnt = 0;
    for c in binary_str.chars().rev() {
        if c == '1' {
            println!("{}", cnt);
            return;
        } else {
            cnt += 1;
        }
    }
}
