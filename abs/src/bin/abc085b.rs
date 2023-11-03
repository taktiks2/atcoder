use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: u32,
    };

    let mut mochis = HashSet::new();

    for _ni in 0..n {
        input! {
            d: u32,
        }
        mochis.insert(d);
    }

    println!("{}", mochis.len());
}
