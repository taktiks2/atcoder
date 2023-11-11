use proconio::input;
use std::cmp::Ordering;

fn main() {
    input! {
        a: i32,
        b: i32,
        c: i32,
        d: i32,
    };

    match (a + b).cmp(&(c + d)) {
        Ordering::Less => println!("Right"),
        Ordering::Equal => println!("Balanced"),
        Ordering::Greater => println!("Left"),
    }
}
