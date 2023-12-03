use proconio::input;

fn main() {
    input! {
        _: usize,
        s: String,
    };

    let mut a_flag = false;
    let mut b_flag = false;
    let mut c_flag = false;
    for (i, c) in s.chars().enumerate() {
        match c {
            'A' => a_flag = true,
            'B' => b_flag = true,
            'C' => c_flag = true,
            _ => (),
        }
        if a_flag && b_flag && c_flag {
            println!("{}", i + 1);
            return;
        }
    }
}
