use proconio::input;
use std::cmp::Ordering;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [usize; m],
    };

    for i in 1..=n {
        match i.cmp(&a[0]) {
            Ordering::Less => {
                println!("{}", a[0] - i);
            }
            Ordering::Equal => {
                println!("0");
            }
            Ordering::Greater => {
                a.remove(0);
                println!("{}", a[0] - i);
            }
        }
    }
}
