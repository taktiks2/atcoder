use proconio::input;

fn main() {
    input! {
        n: usize,
    };

    let mut x_ttl = 0;
    let mut y_ttl = 0;

    for _ in 0..n {
        input! {
            x: usize,
            y: usize,
        }

        x_ttl += x;
        y_ttl += y;
    }

    match x_ttl.cmp(&y_ttl) {
        std::cmp::Ordering::Greater => println!("Takahashi"),
        std::cmp::Ordering::Less => println!("Aoki"),
        std::cmp::Ordering::Equal => println!("Draw"),
    }
}
