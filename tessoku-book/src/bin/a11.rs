use proconio::input;
use std::cmp::Ordering;

fn search(arr: &[i32], target: i32) -> Option<usize> {
    let mut l = 0;
    let mut r = arr.len();

    while l <= r {
        let m = (l + r) / 2;
        match arr[m].cmp(&target) {
            Ordering::Less => l = m + 1,
            Ordering::Equal => return Some(m),
            Ordering::Greater => r = m - 1,
        }
    }
    None
}

fn main() {
    input! {
        n: usize,
        x: i32,
        a: [i32; n]
    };

    match search(&a, x) {
        Some(i) => println!("{}", i + 1),
        None => println!("There is no target"),
    }
}
