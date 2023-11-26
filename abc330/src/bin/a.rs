use proconio::input;

fn main() {
    input! {
        n: usize,
        l: i32,
        a: [i32; n],
    };

    let mut cnt = 0;

    for ai in a {
        if ai >= l {
            cnt += 1;
        }
    }

    println!("{}", cnt)
}
