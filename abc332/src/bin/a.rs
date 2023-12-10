use proconio::input;

fn main() {
    input! {
        n: usize,
        s: usize,
        k: usize,
    };

    let mut ttl = 0;
    for _ in 0..n {
        input! {
            p: usize,
            q: usize,
        };
        ttl += p * q;
    }

    if ttl < s {
        ttl += k;
    }

    println!("{}", ttl);
}
