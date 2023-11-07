use proconio::input;

fn calc_gcd(m: i64, n: i64) -> i64 {
    if n == 0 {
        m
    } else {
        calc_gcd(n, m % n)
    }
}

fn main() {
    input! {
        a: i64,
        b: i64,
        c: i64,
    };

    let mut ab_gcd = 0;
    if a > b {
        ab_gcd = calc_gcd(a, b);
    } else {
        ab_gcd = calc_gcd(b, a);
    }

    let mut abc_gcd = 0;
    if ab_gcd > c {
        abc_gcd = calc_gcd(ab_gcd, c);
    } else {
        abc_gcd = calc_gcd(c, ab_gcd);
    }

    println!("{}", a / abc_gcd + b / abc_gcd + c / abc_gcd - 3)
}
