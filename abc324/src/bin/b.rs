use proconio::input;

fn main() {
    input! {
        n: u64,
    };

    let two = (n as f64).sqrt().ceil() as u32;
    let three = (n as f64).cbrt().ceil() as u32;

    for i in 0..two {
        let pow2 = 2u64.pow(i);
        for j in 0..three {
            let pow3 = 3u64.pow(j);
            if pow2 > n / pow3 {
                break;
            }
            if pow2 * pow3 == n {
                println!("Yes");
                return;
            }
        }
    }

    println!("No");
}
