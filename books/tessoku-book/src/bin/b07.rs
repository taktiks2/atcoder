use proconio::input;

fn main() {
    input! {
        t: usize,
        n: usize,
        lr: [(usize, usize); n],
    };

    let mut arr: Vec<i32> = vec![0; t + 2];

    for (l, r) in lr {
        arr[l + 1] += 1;
        arr[r + 1] -= 1
    }

    let mut acc = vec![0; t + 1];
    for i in 1..=t {
        acc[i] = acc[i - 1] + arr[i];
        println!("{}", acc[i])
    }
}
