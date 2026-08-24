use proconio::input;

fn main() {
    input! {
        a: u64,
        b: u32,
        c: u64,
    };

    let mut result: u64 = 1;
    for _ in 1..=b {
        result *= c;
    }

    if a < result {
        println!("Yes");
    } else {
        println!("No");
    }
}
