use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
        q: usize,
        x: [usize; q]
    };

    a.sort();

    x.iter().for_each(|x| match a.binary_search(x) {
        Ok(index) => println!("{}", index),
        Err(index) => println!("{}", index),
    })
}
