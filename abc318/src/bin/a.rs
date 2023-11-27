use proconio::input;

fn main() {
    input! {
        n: i32,
        m: i32,
        p: i32,
    };

    let remain = n - m;

    if remain < 0 {
        println!("0");
        return;
    }

    println!("{}", (n - m) / p + 1);
}
