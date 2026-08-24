use proconio::input;

fn gcd(a: u128, b: u128) -> u128 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: u128, b: u128) -> u128 {
    a / gcd(a, b) * b
}

fn main() {
    input! {
        a: u128,
        b: u128,
    };

    let ans = lcm(a, b);

    if ans > 1_000_000_000_000_000_000 {
        println!("Large")
    } else {
        println!("{}", ans)
    }
}
