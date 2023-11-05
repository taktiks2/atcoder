use proconio::input;

fn main() {
    input! {
        b: u64,
    };

    let mut a: u32 = 1;

    loop {
        let mut result = 1;

        for _ in 0..a {
            result *= a as u64;
        }

        if result > b {
            println!("-1");
            return;
        } else if result == b {
            println!("{}", a);
            return;
        }

        a += 1;
    }
}
