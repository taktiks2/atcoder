use proconio::input;

fn main() {
    input! {
        n: usize,
        a: i32,
        b: i32,
        c: [i32; n],
    };

    let index = c
        .iter()
        .enumerate()
        .find(|&(_, &val)| val == a + b)
        .map(|(idx, _)| idx);

    match index {
        Some(idx) => println!("{}", idx + 1),
        None => println!("-1"),
    }
}
