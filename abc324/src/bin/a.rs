use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i32; n],
    };

    for i in 1..n {
        if a[i] != a[0] {
            println!("No");
            return;
        }
    }
    println!("Yes")
}
