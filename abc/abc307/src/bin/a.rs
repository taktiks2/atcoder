use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i32; n * 7],
    };

    for i in 0..n {
        let mut total = 0;
        for j in 0..7 {
            total += a[i * 7 + j];
        }
        if i == n - 1 {
            println!("{}", total);
        } else {
            print!("{} ", total)
        }
    }
}
