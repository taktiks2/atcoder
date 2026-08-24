use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [i32; n]
    };

    a.sort_by(|a, b| b.cmp(a));

    for i in 0..n - 1 {
        if a[i] != a[i + 1] {
            println!("{}", a[i + 1]);
            break;
        }
    }
}
