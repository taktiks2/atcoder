use proconio::input;

fn main() {
    input! {
        n: i32,
        ttl: i32,
    };

    let mut ans = String::from("-1 -1 -1");

    for x in 0..=n {
        for y in 0..=(n - x) {
            let z = n - x - y;
            if x * 10000 + y * 5000 + z * 1000 == ttl {
                ans = format!("{} {} {}", x, y, z);
            }
        }
    }

    println!("{}", ans);
}
