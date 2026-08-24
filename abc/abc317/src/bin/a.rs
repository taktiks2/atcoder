use proconio::input;

fn main() {
    input! {
        n: usize,
        h: i32,
        x: i32,
        p: [i32; n],
    };

    for (i, pe) in p.iter().enumerate() {
        if h + pe >= x {
            println!("{}", i + 1);
            return;
        }
    }
}
