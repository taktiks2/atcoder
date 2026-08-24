use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [i32; n],
    };

    let mut cnt = 0;

    while a.iter().all(|&x| x % 2 == 0) {
        a = a.iter().map(|&x| x / 2).collect();
        cnt += 1;
    }
    println!("{}", cnt);
}
