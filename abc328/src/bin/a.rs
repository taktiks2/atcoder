use proconio::input;

fn main() {
    input! {
        n: i32,
        x: i32,
        s: [i32; n],
    };

    let ans: i32 = s.iter().filter(|&num| num <= &x).sum();

    println!("{}", ans);
}
