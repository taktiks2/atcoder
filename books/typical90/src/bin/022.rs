use proconio::input;

fn gcd(m: i64, n: i64) -> i64 {
    if m % n == 0 {
        n
    } else {
        gcd(n, m % n)
    }
}

fn main() {
    input! {
        a: i64,
        b: i64,
        c: i64,
    };

    let g = gcd(gcd(a, b), c);

    println!("{}", (a + b + c) / g - 3)
}
