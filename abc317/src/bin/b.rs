use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [i32; n],
    };

    a.sort();

    for i in 0..n - 1 {
        if a[i + 1] - a[i] != 1 {
            println!("{}", a[i] + 1);
            return;
        }
    }
}
