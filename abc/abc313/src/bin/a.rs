use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [usize; n],
    };

    let mut subs = vec![];
    for i in 1..n {
        if p[0] <= p[i] {
            subs.push(p[i] - p[0])
        }
    }

    let ans = match subs.len() {
        0 => 0,
        _ => *subs.iter().max().unwrap() + 1,
    };

    println!("{}", ans)
}
