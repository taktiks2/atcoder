use proconio::input;

fn check(arr: &[usize], mid: usize, target: usize) -> bool {
    let mut sum = 0;
    arr.iter().for_each(|&x| sum += mid / x);
    sum >= target
}

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n]
    };

    let mut left = 0;
    let mut right = 1_000_000_000;

    while left < right {
        let mid = (left + right) / 2;
        match check(&a, mid, k) {
            true => right = mid,
            false => left = mid + 1,
        }
    }
    println!("{}", left)
}
