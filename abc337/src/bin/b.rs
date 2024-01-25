use proconio::input;

fn create_str(c: char, n: usize) -> String {
    std::iter::repeat(c).take(n).collect()
}

fn main() {
    input! {
        s: String,
    };

    let len = s.len();

    for a in 0..=len {
        for b in 0..=len {
            for c in 0..=len {
                let str = create_str('A', a) + &create_str('B', b) + &create_str('C', c);
                if str == s {
                    println!("Yes");
                    return;
                }
            }
        }
    }
    println!("No");
}
