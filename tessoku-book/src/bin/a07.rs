use proconio::input;

fn main() {
    input! {
        d: usize,
        n: usize,
        lr: [(usize, usize); n],
    };

    let mut schedule: [i32; 100002] = [0; 100002];
    for (l, r) in lr {
        schedule[l] += 1;
        schedule[r + 1] -= 1;
    }

    let mut acc: Vec<i32> = vec![];
    acc.push(0);
    for i in 1..=d {
        acc.push(acc[i - 1] + schedule[i])
    }

    for i in 1..=d {
        println!("{}", acc[i]);
    }
}
