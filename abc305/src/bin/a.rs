use proconio::input;

fn main() {
    input! {
        n: i32,
    };

    let numbers: Vec<i32> = (0..=100).step_by(5).collect();
    let ans = numbers
        .iter()
        .min_by_key(|&&x| (x - n).abs())
        .copied()
        .unwrap();
    println!("{}", ans)
}
