use proconio::input;

fn main() {
    input! {
        n: usize,
        d: [i32; n],
    };

    let mut total = 0;

    for i in 1..=n {
        for j in 1..=d[i - 1] {
            let num = format!("{}{}", i, j);

            if num.chars().all(|c| c == num.chars().nth(0).unwrap()) {
                total += 1;
            }
        }
    }

    println!("{}", total)
}
