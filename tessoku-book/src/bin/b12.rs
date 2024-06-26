use proconio::input;

fn check(mid: f64, target: f64) -> bool {
    let x = mid.powi(3) + mid;
    if x <= target {
        return true;
    }
    false
}

fn main() {
    input! {
        n: f64,
    };

    let mut left: f64 = 0.0;
    let mut right: f64 = 100.0;

    while right - left > 0.001 {
        let mid: f64 = (left + right) / 2.0;
        match check(mid, n) {
            true => left = mid,
            false => right = mid,
        }
    }

    println!("{}", left)
}
